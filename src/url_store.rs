use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::{Pool, Sqlite};

use crate::{cache::TtlCache};

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

    pub async fn get(&self, key: String) -> Result<Option<String>, sqlx::Error> {
        //self.stats_tx.send(key.clone()).await.map_err(|e| e.to_string())?;
        let value = if let Some(maybe_url) = self.cache.get(&key).await {
            maybe_url
        } else {
            //run db query to get the value
            let value = if let Some(row) =
                sqlx::query!("SELECT longurl from shorturls WHERE shorturl = ?", key)
                    .fetch_optional(&self.sqlite_pool)
                    .await
                   ?
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

    pub async fn get_all(&self) -> Result<Vec<ShortUrlRow>, sqlx::Error> {
         sqlx::query_as!(ShortUrlRow,
            "SELECT * FROM shorturls ORDER BY created_at DESC"
        ).fetch_all(&self.sqlite_pool)
            .await
    }

     fn generate_short_url(&self) -> String {
        nanoid::nanoid!(8)
    }
}

#[derive(Debug, Clone , sqlx::FromRow)]
pub struct ShortUrlRow {
   pub shorturl: String,
   pub longurl: String,
   pub created_at: NaiveDateTime,
}