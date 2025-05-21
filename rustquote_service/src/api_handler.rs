//! # API Handlers
//!
//! This module contains the Axum handlers for the API endpoints.
//! It defines the logic for responding to HTTP requests for health checks and quote retrieval.

use crate::services::quote_service;
use crate::AppState;
use axum::{extract::State, http::StatusCode, Json}; // Added State // Import AppState
                                                    // Quote model is not directly used here anymore for response construction, but might be for logic
                                                    // use crate::models::Quote;
use crate::errors::AppError;
use crate::responses::{HealthStatus, QuoteResponse}; // ErrorResponse is now handled by AppError // Import the custom error type

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
pub async fn get_quote_handler(
    State(app_state): State<AppState>, // Extract AppState
) -> Result<Json<QuoteResponse>, AppError> {
    tracing::debug!(
        "Received request for /api/v1/quote. Using quotes_file_path: {}",
        app_state.quotes_file_path.display()
    );
    match quote_service::load_quotes_from_file(&app_state.quotes_file_path) {
        Ok(quotes) => {
            if quotes.is_empty() {
                // Explicitly check for empty quotes vector
                Err(AppError::NotFound(
                    "No quotes available in the data file.".to_string(),
                ))
            } else if let Some(random_quote) = quote_service::get_random_quote(&quotes) {
                let response = QuoteResponse {
                    id: random_quote.id,
                    quote: random_quote.text.clone(),
                    author: random_quote.author.clone(),
                };
                tracing::info!("Successfully retrieved and returned a random quote.");
                Ok(Json(response))
            } else {
                // This branch should ideally not be hit if quotes.is_empty() is checked above
                // but kept for safety, or if get_random_quote could fail for other reasons on non-empty.
                Err(AppError::NotFound(
                    // Should be unreachable if quotes.is_empty() is handled
                    "Could not select a random quote (unexpected).".to_string(),
                ))
            }
        }
        Err(service_error) => {
            // Log the specific service_error for better diagnostics
            tracing::error!(
                "Quote service error during load_quotes_from_file: {:?}",
                service_error
            ); // Changed to debug print
            tracing::error!("Failed to load quotes (display): {}", service_error);
            // Convert QuoteServiceError to AppError using the From trait
            Err(AppError::from(service_error))
        }
    }
}
/// Handles requests to the `/api/v1/quote/:id` endpoint.
///
/// Attempts to load quotes from the configured data file, finds the quote by ID,
/// and returns it in a JSON response.
///
/// # Errors
///
/// Returns an [`AppError::NotFound`] if the quote with the specified ID is not found or no quotes are available.
/// Returns an [`AppError::InternalServerError`] or [`AppError::QuoteSourcingError`] if there's an issue loading quotes.
pub async fn get_quote_by_id_handler(
    State(app_state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> Result<Json<QuoteResponse>, AppError> {
    tracing::debug!(
        "Received request for /api/v1/quote/{}. Using quotes_file_path: {}",
        id,
        app_state.quotes_file_path.display()
    );

    match quote_service::load_quotes_from_file(&app_state.quotes_file_path) {
        Ok(quotes) => {
            if quotes.is_empty() {
                tracing::warn!(
                    "No quotes available in the data file when searching for ID: {}",
                    id
                );
                return Err(AppError::NotFound(format!(
                    "No quotes available in the data file. Cannot find quote with ID: {}.",
                    id
                )));
            }

            if let Some(quote) = quote_service::get_quote_by_id(&quotes, id) {
                let response = QuoteResponse {
                    id: quote.id,
                    quote: quote.text.clone(),
                    author: quote.author.clone(),
                };
                tracing::info!("Successfully retrieved and returned quote with ID: {}", id);
                Ok(Json(response))
            } else {
                tracing::info!("Quote with ID: {} not found.", id);
                Err(AppError::NotFound(format!(
                    "Quote with ID: {} not found.",
                    id
                )))
            }
        }
        Err(service_error) => {
            tracing::error!(
                "Quote service error during load_quotes_from_file for ID {}: {:?}",
                id,
                service_error
            );
            tracing::error!("Failed to load quotes (display): {}", service_error);
            Err(AppError::from(service_error)) // Converts QuoteServiceError to AppError
        }
    }
}
