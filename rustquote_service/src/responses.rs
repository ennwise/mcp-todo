//! # API Response Structures
//!
//! This module defines the standard structures used for serializing API responses.
//! These structs are used by the API handlers to provide consistent JSON outputs.

use serde::{Serialize, Deserialize}; // Add Deserialize

/// Represents the health status of the service.
///
/// Used as the response for the `/api/health` endpoint.
#[derive(Serialize)]
pub struct HealthStatus {
    /// A string indicating the current status, e.g., "healthy".
    pub status: String,
}

/// Represents a quote response.
///
/// Used as the success response for the `/api/v1/quote` endpoint.
#[derive(Serialize, Deserialize)] // Add Deserialize
pub struct QuoteResponse {
    /// The text of the quote.
    pub quote: String,
    /// The author of the quote.
    pub author: String,
}

/// Represents a standardized error response for the API.
///
/// This structure is used to provide consistent error messages to clients.
/// Note: This is currently superseded by `AppError`'s `IntoResponse` implementation
/// for direct error handling in Axum, but kept for potential future use or reference.
#[derive(Serialize)]
pub struct ErrorResponse {
    /// A machine-readable error code.
    pub error_code: String,
    /// A human-readable message describing the error.
    pub message: String,
}