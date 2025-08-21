use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::RwLock, task::JoinHandle};

#[derive(Debug, Clone)]
struct CacheEntry {
    value: String,
    expiry: Instant,
}

#[derive(Clone)]
pub struct TtlCache {
    map: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl: Duration,
}

impl TtlCache {
    /// Create a new cache and start the cleaner immediately
    pub async fn new(ttl: Duration, cleanup_interval: Duration) -> (Self, JoinHandle<()>) {
        let cache = Self {
            map: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        };

        let cleaner = cache._spawn_cleaner(cleanup_interval);
        (cache, cleaner)
    }

    pub async fn insert(&self, key: String, value: String) {
        let expiry = Instant::now() + self.ttl;
        let entry = CacheEntry { value, expiry };
        self.map.write().await.insert(key, entry);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        let mut map = self.map.write().await;
        if let Some(entry) = map.get_mut(key) {
            let now = Instant::now();
            if now <= entry.expiry {
                // sliding TTL: reset expiry
                entry.expiry = now + self.ttl;
                return Some(entry.value.clone());
            } else {
                map.remove(key);
            }
        }
        None
    }

    /// Internal: spawn the background cleaner
    fn _spawn_cleaner(&self, interval: Duration) -> JoinHandle<()> {
        let map = self.map.clone();

        let handle = tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                let mut map = map.write().await;
                let now = Instant::now();
                map.retain(|_, entry| entry.expiry > now);
                println!("🧹 Cache cleaned, {} keys remain", map.len());
            }
        });

        handle
    }
}
