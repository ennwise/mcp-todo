//! # Application Errors
//!
//! This module defines the primary error type (`AppError`) for the application,
//! along with conversions from other error types and its `IntoResponse` implementation
//! for Axum to automatically convert `AppError` into HTTP responses.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

// Re-using the ErrorResponse struct from responses.rs for consistency
// If responses.rs is not in the same module, you might need to adjust the path
// For example, if responses is a sibling module: use crate::responses::ErrorResponse;
// Assuming responses.rs is in the same module or crate root for now.
use crate::responses::ErrorResponse;
use crate::services::quote_service::QuoteServiceError; // Added import

/// Represents all possible errors that can occur within the application.
///
/// This enum uses `thiserror` to derive `std::error::Error` and `std::fmt::Display`.
/// It also implements `IntoResponse` to allow Axum to convert these errors
/// directly into HTTP responses.
#[derive(Error, Debug)]
pub enum AppError {
    /// Represents an unexpected internal server error.
    /// Contains a message describing the internal error.
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),

    /// Represents a situation where a requested resource was not found.
    /// Contains a message describing what was not found.
    #[error("Not Found: {0}")]
    NotFound(String),

    /// Represents an error due to invalid client input.
    /// Contains a message describing the nature of the bad request.
    #[error("Bad Request: {0}")]
    BadRequest(String),

    /// Represents an error specifically related to sourcing quotes (e.g., file I/O, parsing).
    /// Contains a message describing the sourcing issue.
    #[error("Quote Sourcing Error: {0}")]
    QuoteSourcingError(String),
    // Add other specific error types as needed
}

/// Converts an [`AppError`] into an Axum `Response`.
///
/// This implementation maps each variant of `AppError` to an appropriate
/// HTTP status code and a JSON error response body.
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

/// Converts a `std::io::Error` into an [`AppError::QuoteSourcingError`].
///
/// This is a convenience implementation for handling I/O errors that occur
/// during quote sourcing operations.
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

/// Converts a [`QuoteServiceError`] into an [`AppError`].
///
/// This allows errors from the `quote_service` module to be easily propagated
/// and handled as a more general `AppError`, typically as a `QuoteSourcingError`.
impl From<QuoteServiceError> for AppError {
    fn from(err: QuoteServiceError) -> Self {
        match err {
            QuoteServiceError::FileNotFound(path) => {
                AppError::QuoteSourcingError(format!("Quote data file not found: {}", path))
            }
            QuoteServiceError::FileReadError(io_err) => {
                AppError::QuoteSourcingError(format!("Error reading quote data file: {}", io_err))
            }
            QuoteServiceError::ParseError(parse_err) => {
                AppError::QuoteSourcingError(format!("Error parsing quote data: {}", parse_err))
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;
    use serde_json::json;
    use crate::services::quote_service::QuoteServiceError; // Ensure this is available

    #[tokio::test]
    async fn test_app_error_internal_server_error_into_response() {
        let error = AppError::InternalServerError("Test internal error".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let expected_json = json!({
            "error_code": "INTERNAL_SERVER_ERROR",
            "message": "Test internal error"
        });
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&body).unwrap(), expected_json);
    }

    #[tokio::test]
    async fn test_app_error_not_found_into_response() {
        let error = AppError::NotFound("Resource not here".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let expected_json = json!({
            "error_code": "NOT_FOUND",
            "message": "Resource not here"
        });
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&body).unwrap(), expected_json);
    }

    #[tokio::test]
    async fn test_app_error_bad_request_into_response() {
        let error = AppError::BadRequest("Bad input".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let expected_json = json!({
            "error_code": "BAD_REQUEST",
            "message": "Bad input"
        });
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&body).unwrap(), expected_json);
    }

    #[tokio::test]
    async fn test_app_error_quote_sourcing_error_into_response() {
        let error = AppError::QuoteSourcingError("Could not get quotes".to_string());
        let response = error.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // As per current impl
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let expected_json = json!({
            "error_code": "QUOTE_SOURCING_ERROR",
            "message": "Could not get quotes"
        });
        assert_eq!(serde_json::from_slice::<serde_json::Value>(&body).unwrap(), expected_json);
    }

    #[test]
    fn test_from_io_error_for_app_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_error: AppError = io_error.into();
        match app_error {
            AppError::QuoteSourcingError(msg) => {
                assert_eq!(msg, "IO error: file not found");
            }
            _ => panic!("Incorrect AppError variant"),
        }
    }

    #[test]
    fn test_from_quote_service_error_file_not_found() {
        let q_error = QuoteServiceError::FileNotFound("quotes.json".to_string());
        let app_error: AppError = q_error.into();
        match app_error {
            AppError::QuoteSourcingError(msg) => {
                assert_eq!(msg, "Quote data file not found: quotes.json");
            }
            _ => panic!("Incorrect AppError variant"),
        }
    }

    #[test]
    fn test_from_quote_service_error_file_read_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "cannot read");
        let q_error = QuoteServiceError::FileReadError(io_err);
        let app_error: AppError = q_error.into();
        match app_error {
            AppError::QuoteSourcingError(msg) => {
                assert_eq!(msg, "Error reading quote data file: cannot read");
            }
            _ => panic!("Incorrect AppError variant"),
        }
    }

    #[test]
    fn test_from_quote_service_error_parse_error() {
        // Create a dummy serde_json::Error
        // This is a bit tricky as serde_json::Error is not easily constructible with a simple message.
        // We'll simulate it by parsing invalid JSON.
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let q_error = QuoteServiceError::ParseError(json_err);
        let app_error: AppError = q_error.into();
        match app_error {
            AppError::QuoteSourcingError(msg) => {
                // The exact message from serde_json::Error can be complex.
                // We'll check if it contains the expected prefix.
                assert!(msg.starts_with("Error parsing quote data:"));
            }
            _ => panic!("Incorrect AppError variant"),
        }
    }

    // Test Display trait (implicitly via thiserror)
    #[test]
    fn test_app_error_display() {
        assert_eq!(
            AppError::InternalServerError("db down".to_string()).to_string(),
            "Internal Server Error: db down"
        );
        assert_eq!(
            AppError::NotFound("item 123".to_string()).to_string(),
            "Not Found: item 123"
        );
        assert_eq!(
            AppError::BadRequest("missing field".to_string()).to_string(),
            "Bad Request: missing field"
        );
        assert_eq!(
            AppError::QuoteSourcingError("file access".to_string()).to_string(),
            "Quote Sourcing Error: file access"
        );
    }
}