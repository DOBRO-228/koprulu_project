use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use log::error;
use sea_orm::DbErr;
use serde::Serialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AppError {
    DatabaseError(String),
    NotFound(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            // Handle other variants
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
        };
        let error_response = Json(json!({
            "error": self.to_string()
        }));

        (status_code, error_response).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(e: DbErr) -> Self {
        let error = e.to_string();
        error!("{:?}, {}", e, error);
        AppError::DatabaseError(error)
    }
}
