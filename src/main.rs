use crate::{cache::TtlCache, url_store::UrlStore};
use axum::{
    debug_handler, extract::{Path, State}, http::StatusCode, response::{Html, IntoResponse, Redirect, Response}, Form
};
use std::{env, sync::Arc, time::Duration};
use tokio::signal::ctrl_c;

mod cache;
mod url_store;
#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber for logging
    //TODO configure the tracing subscriber to support tokio console as well as env filter
    tracing_subscriber::fmt().init();

    //load the environment variables from the .env file
    dotenvy::dotenv().ok();

    // Initialize the SQLite connection pool
    let sqlite_pool =
        sqlx::SqlitePool::connect(&env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
            .await
            .expect("Failed to create SQLite pool");

    // Initialize the cache with a TTL of 60 seconds and a cleanup interval of 10 seconds
    let cache = TtlCache::new(Duration::from_secs(60), Duration::from_secs(10));

    let url_store = url_store::UrlStore::new(sqlite_pool.clone(), cache.clone()).await;

    let router = axum::Router::new()
        .route("/", axum::routing::get(get_hompeage))
        .route("/add", axum::routing::post(post_add_url))
        .route("/{s}", axum::routing::get(get_redirect_to_url))
        .with_state(url_store);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");

    // Stop the cache cleaner and close the SQLite pool gracefully
    cache.stop_cleaner().await;
    sqlite_pool.close().await;
    println!("Server has been shut down gracefully.");
}

async fn shutdown_signal() {
    ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");
}

async fn get_hompeage(State(u): State<UrlStore>) -> Response {
    let values = u.get_all().await.unwrap();
    let table = if values.is_empty() {
        "no shortened URLs found".to_string()
    } else {
        let rows = values
            .into_iter()
            .map(|(short, long, c)| {
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                    short,
                    long,
                    c.to_rfc3339()
                )
            })
            .reduce(|a, b| a + &b)
            .unwrap_or_default();
        format!(
            r#"
        <table>
            <thead>
                <tr>
                    <th>Short URL</th>
                    <th>Long URL</th> 
                    <th>Created At</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
    "#,
            rows
        )
    };
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>URL Shortener</title>
        </head>
        <body>
            <h1>Welcome to the URL Shortener</h1>
            <form action="/add" method="post">
                <input type="text" name="url" placeholder="Enter URL" required>
                <button type="submit">Shorten</button>
            </form>
            <br />
                {}
        </body>
        </html>
        "#,
        table
    );

    (StatusCode::OK, Html(html)).into_response()
}


#[derive(Debug, serde::Deserialize)]
struct AddUrlForm {
    url: String,
}

async fn post_add_url(
    State(u): State<UrlStore>,
    Form(AddUrlForm { url }): Form<AddUrlForm>,
) -> Response {
    if let Err(e) = u.insert(url).await {
        tracing::error!("Error inserting URL: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("error: {e}")).into_response()
    } else {
        Redirect::to("/").into_response()
    }
}
#[debug_handler]
async fn get_redirect_to_url(Path(s): Path<String>, State(u): State<UrlStore>) -> Response {
    match u.get(s).await {
        Ok(Some(url)) => {
            tracing::info!("Redirecting to URL: {}", url);
            axum::response::Redirect::to(&url).into_response()
        }
        Ok(None) => {
            tracing::warn!("URL not found");
            StatusCode::NOT_FOUND.into_response()
        }
        Err(e) => {
            tracing::error!("Error retrieving URL: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("error: {e}")).into_response()
        }
    }
}
