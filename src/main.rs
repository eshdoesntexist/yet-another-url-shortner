use crate::{
    cache::TtlCache,
    errors::{AppError, AppResult},
    partials::{page, url_table},
    url_store::UrlStore,
};
use axum::{
    Form, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use std::{env, time::Duration};
use tokio::signal::ctrl_c;

mod cache;
mod errors;
mod partials;
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
    let (cache, cleaner_handle) =
        TtlCache::new(Duration::from_secs(60), Duration::from_secs(10)).await;

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

    //close the SQLite pool gracefully
    sqlite_pool.close().await;
    cleaner_handle.abort();
    println!("Server has been shut down gracefully.");
}

async fn shutdown_signal() {
    ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");
}

async fn get_hompeage(State(u): State<UrlStore>) -> Response {
    let values = u.get_all().await.unwrap();
    let homepage = page("Home", url_table(values));

    (StatusCode::OK, homepage).into_response()
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
async fn get_redirect_to_url(Path(s): Path<String>, State(u): State<UrlStore>) -> AppResult {
    match u.get(s).await? {
        Some(url) => {
            tracing::info!("Redirecting to URL: {}", url);
            Ok(axum::response::Redirect::to(&url).into_response())
        }
        None => {
            tracing::warn!("URL not found");
            Err(AppError::custom(StatusCode::NOT_FOUND, "Url not found"))
        }
    }
}
