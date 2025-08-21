use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::html;

use crate::partials::page;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Sqlite/Sqlx error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("{msg}")]
    CustomError { code: StatusCode, msg: String },
}

impl AppError {
    pub fn custom(code: StatusCode, msg: impl ToString) -> Self {
        Self::CustomError {
            code,
            msg: msg.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            
            AppError::DatabaseError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::CustomError { code, .. } => code,
        };
        let error_page = html! {
            main
                class="grid min-h-full place-items-center bg-gray-900 px-6 py-24 sm:py-32 lg:px-8"
            {
                div class="text-center" {
                    p class="text-base font-semibold text-indigo-400" { (status.to_string()) }
                    h1
                        class="mt-4 text-5xl font-semibold tracking-tight text-balance text-white sm:text-7xl"
                    { "Page not found" }
                    p class="mt-6 text-lg font-medium text-pretty text-gray-400 sm:text-xl/8" {
                        "Sorry, we couldn't find the page you're looking for."
                    }
                    div class="mt-10 flex items-center justify-center gap-x-6" {
                        a
                            href="/"
                            class="rounded-md bg-indigo-500 px-3.5 py-2.5 text-sm font-semibold text-white shadow-xs hover:bg-indigo-400 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
                        { "Go back home" }
                    }
                }
            }
        };
        let doc = page("Error", error_page);
        (status, doc).into_response()
    }
}

pub type AppResult = Result<Response, AppError>;
