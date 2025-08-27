use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::views::ErrorPage;

#[derive(Debug, thiserror::Error)]
//#[non_exhaustive]
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
        tracing::error!("Error occurred: {}", self);
        let status = match self {
            AppError::DatabaseError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppError::CustomError { code, .. } => code,
            // _ => StatusCode::INTERNAL_SERVER_ERROR
        };
        ErrorPage::new()
            .set_status(status)
            .set_message(self.to_string())
            .into_response()
    }
}

pub type AppResult = Result<Response, AppError>;
