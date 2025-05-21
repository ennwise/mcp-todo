//! # API Handlers
//!
//! This module contains the Axum handlers for the API endpoints.
//! It defines the logic for responding to HTTP requests for health checks and quote retrieval.

use axum::{http::StatusCode, Json};
use crate::services::quote_service;
// Quote model is not directly used here anymore for response construction, but might be for logic
// use crate::models::Quote;
use crate::responses::{HealthStatus, QuoteResponse}; // ErrorResponse is now handled by AppError
use crate::errors::AppError; // Import the custom error type

/// Handles requests to the `/api/health` endpoint.
///
/// Returns a JSON response indicating the service is healthy.
/// This endpoint can be used for liveness/readiness probes.
pub async fn health_check_handler() -> (StatusCode, Json<HealthStatus>) {
    // No input parameters or body to validate for this endpoint.
    // If there were, validation logic would go here.
    let health = HealthStatus {
        status: "healthy".to_string(),
    };
    (StatusCode::OK, Json(health))
}

/// Handles requests to the `/api/v1/quote` endpoint.
///
/// Attempts to load quotes from the configured data file, selects a random quote,
/// and returns it in a JSON response.
///
/// # Errors
///
/// Returns an [`AppError::NotFound`] if no quotes are available.
/// Returns an [`AppError::InternalServerError`] if there's an issue loading quotes.
pub async fn get_quote_handler() -> Result<Json<QuoteResponse>, AppError> {
    tracing::debug!("Received request for /api/v1/quote");
    // TODO: Make the quotes file path configurable
    match quote_service::load_quotes_from_file("data/quotes.json") {
        Ok(quotes) => {
            if let Some(random_quote) = quote_service::get_random_quote(&quotes) {
                let response = QuoteResponse {
                    quote: random_quote.text.clone(),
                    author: random_quote.author.clone(),
                };
                tracing::info!("Successfully retrieved and returned a random quote.");
                Ok(Json(response))
            } else {
                // Use AppError for no quotes available
                Err(AppError::NotFound(
                    "No quotes available in the data file.".to_string(),
                ))
            }
        }
        Err(service_error) => {
            tracing::error!("Failed to load quotes: {}", service_error);
            // Convert QuoteServiceError to AppError using the From trait
            Err(AppError::from(service_error))
        }
    }
}