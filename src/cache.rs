use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};
use axum::http::header::CACHE_CONTROL;
use tokio::{sync::RwLock, task::JoinHandle};

use crate::cache;

#[derive(Debug, Clone)]
struct CacheEntry {
    value: String,
    expiry: Instant,
}

#[derive(Clone)]
pub struct TtlCache {
    map: Arc<RwLock<HashMap<String, CacheEntry>>>,
    ttl: Duration,
    stop_flag: Arc<AtomicBool>,
    cleaner_handle: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl TtlCache {
    /// Create a new cache and start the cleaner immediately
    pub fn new(ttl: Duration, cleanup_interval: Duration) -> Self {
        let cache = Self {
            map: Arc::new(RwLock::new(HashMap::new())),
            ttl,
            stop_flag: Arc::new(AtomicBool::new(false)),
            cleaner_handle: Arc::new(RwLock::new(None)),
        };
        let cache_clone = cache.clone();
       tokio::spawn(async move {
            // Spawn cleaner as part of construction
            let mut lock = cache_clone.cleaner_handle.write().await;
            *lock = Some(cache_clone._spawn_cleaner(cleanup_interval));
        });
        cache
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
        let stop_flag = self.stop_flag.clone();

        tokio::spawn(async move {
            loop {
                tokio::time::sleep(interval).await;
                if stop_flag.load(Ordering::Relaxed) {
                    println!("🛑 Cache cleaner stopped");
                    break;
                }
                let mut map = map.write().await;
                let now = Instant::now();
                map.retain(|_, entry| entry.expiry > now);
                println!("🧹 Cache cleaned, {} keys remain", map.len());
            }
        })
    }

    /// Stop the cleaner gracefully
    pub async fn stop_cleaner(&self) {
        self.stop_flag.store(true, Ordering::Relaxed);
        if let Some(handle) = self.cleaner_handle.write().await.take() {
            let _ = handle.await;
        }
    }
}
