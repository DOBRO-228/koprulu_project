use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use log::error;
use sea_orm::DbErr;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DbError(DbErr),
    OtherError(String),
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error_message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DbError(e) => write!(f, "Database error: {}", e),
            AppError::OtherError(e) => write!(f, "Error: {}", e),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        // Log the error centrally
        error!("{}", err);
        AppError::DbError(err)
    }
}

impl From<AppError> for (StatusCode, ErrorResponse) {
    fn from(err: AppError) -> Self {
        match err {
            AppError::DbError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_message: err.to_string(),
                },
            ),
            AppError::OtherError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    error_message: err.to_string(),
                },
            ),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error_response = ErrorResponse {
            error_message: self.to_string(),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
    }
}
