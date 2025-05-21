use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

// Re-using the ErrorResponse struct from responses.rs for consistency
// If responses.rs is not in the same module, you might need to adjust the path
// For example, if responses is a sibling module: use crate::responses::ErrorResponse;
// Assuming responses.rs is in the same module or crate root for now.
use crate::responses::ErrorResponse;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Quote Sourcing Error: {0}")]
    QuoteSourcingError(String),
    // Add other specific error types as needed
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message) = match self {
            AppError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_SERVER_ERROR".to_string(),
                msg,
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND".to_string(),
                msg,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST".to_string(),
                msg,
            ),
            AppError::QuoteSourcingError(msg) => (
                // Could be 500 or a more specific client error depending on context
                StatusCode::INTERNAL_SERVER_ERROR,
                "QUOTE_SOURCING_ERROR".to_string(),
                msg,
            ),
        };

        let body = Json(ErrorResponse {
            error_code,
            message,
        });

        (status, body).into_response()
    }
}

// Helper for easy conversion from std::io::Error
impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::QuoteSourcingError(format!("IO error: {}", err))
    }
}

// Example: If using a specific CSV parsing library that has its own error type
// impl From<csv::Error> for AppError {
//     fn from(err: csv::Error) -> Self {
//         AppError::QuoteSourcingError(format!("CSV parsing error: {}", err))
//     }
// }