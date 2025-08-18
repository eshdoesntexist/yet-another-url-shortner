use chrono::{DateTime, Utc};
use sqlx::{Pool, Sqlite, SqliteConnection};

use crate::cache::{self, TtlCache};

#[derive(Clone)]
pub struct UrlStore {
    cache: TtlCache,
    sqlite_pool: Pool<Sqlite>,
    //stats_tx: tokio::sync::mpsc::Sender<String>,
}

impl UrlStore {
    pub async fn new(
        sqlite_pool: Pool<Sqlite>,
        cache: TtlCache,
        //stats_tx: tokio::sync::mpsc::Sender<String>,
    ) -> Self {
        UrlStore {
            cache,
            sqlite_pool,
            //stats_tx,
        }
    }

    pub async fn get(&self, key: String) -> Result<Option<String>, String> {
        //self.stats_tx.send(key.clone()).await.map_err(|e| e.to_string())?;
        let value = if let Some(maybe_url) = self.cache.get(&key).await {
            maybe_url
        } else {
            //run db query to get the value
            let value = if let Some(row) =
                sqlx::query!("SELECT longurl from shorturls WHERE shorturl = ?", key)
                    .fetch_optional(&self.sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())?
            {
                row.longurl
            } else {
                return Ok(None);
            };

            //store the values in cache
            //TODO probably spawn a background task to do this to save some time on the request
            // actually benchmark to check if its worth it
            self.cache.insert(key.clone(), value.clone()).await;
            value
        };
        //TODO add way to report statistics
        return Ok(Some(value));
    }
    pub async fn insert(&self, value: String) -> Result<(), String> {
        let shorturl = self.generate_short_url();
sqlx::query!(
            "INSERT INTO shorturls (shorturl, longurl) VALUES (?, ?)",
            shorturl,
            value
        )
        .execute(&self.sqlite_pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<(String, String, DateTime<Utc>)>, String> {
         sqlx::query!(
            "SELECT * FROM shorturls ORDER BY created_at DESC"
        ).fetch_all(&self.sqlite_pool)
            .await
            .map_err(|e| e.to_string()).and_then(|res| Ok(res.iter().map(| row| {
                (
                    row.shorturl.clone(),
                    row.longurl.clone(),
                    row.created_at.and_utc(),
                )
            }).collect::<Vec<_>>()))
    }

     fn generate_short_url(&self) -> String {
        nanoid::nanoid!(8)
    }
}

