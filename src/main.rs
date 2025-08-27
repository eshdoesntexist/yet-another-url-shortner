use crate::{
    cache::TtlCache,
    errors::{AppError, AppResult},
    url_store::UrlStore,
    views::{DashboardPageBuilder, LoginFormPage, LoginFormPayload},
};
use axum::{
    Form, debug_handler,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::get,
};
use bcrypt::{BcryptError, bcrypt};
use serde::Deserialize;
use std::{env, panic, time::Duration};
use tokio::signal::ctrl_c;
use tower_http::services::ServeDir;

mod cache;
mod errors;
mod handlers;
//mod partials;
mod url_store;
mod views;

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
        .route("/login", get(get_login).post(post_login))
        .route("/{s}", axum::routing::get(get_redirect_to_url))
        .nest_service("/static", ServeDir::new("./static"))
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
    let homepage = DashboardPageBuilder::new().set_rows(values);

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
#[tracing::instrument]
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

#[derive(Deserialize, Debug)]
struct LoginPageQueryParams {
    redirect_to: Option<String>,
}

#[tracing::instrument]
async fn get_login(
    Query(LoginPageQueryParams { redirect_to }): Query<LoginPageQueryParams>,
) -> AppResult {
    Ok(LoginFormPage::new()
        .maybe_redirect_to(redirect_to)
        .into_response())
}

#[tracing::instrument]
async fn post_login(Form(data): Form<LoginFormPayload>) -> AppResult {
    //TODO implement logic
    Ok(LoginFormPage::new()
        .set_prepopulated_email(data.email)
        .maybe_redirect_to(data.redirect_to)
        .show_invalid_credentials()
        .into_response())
}

static HASH_COST: u32 = 10;

async fn compare_pwd<'a>(hash: String, pwd: String) -> Result<bool, BcryptError> {
    tokio::task::spawn_blocking(move || bcrypt::verify(pwd, &hash))
        .await
        .expect("bcrypt either panicked or task was cancelled")
}
async fn hash_pwd(plain_pwd: String) -> Result<String, BcryptError> {
    tokio::task::spawn_blocking(move || bcrypt::hash(plain_pwd, HASH_COST))
        .await
        .expect("bcrypt either panicked or task was cancelled")
}
