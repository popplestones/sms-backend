use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Environment error: {0}")]
    Env(#[from] std::env::VarError),

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message): (StatusCode, &str) = match self {
            AppError::Env(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::Redis(e) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
