use axum::{http::StatusCode, Json};
use crate::services::quote_service;
// Quote model is not directly used here anymore for response construction, but might be for logic
// use crate::models::Quote;
use crate::responses::{HealthStatus, QuoteResponse}; // ErrorResponse is now handled by AppError
use crate::errors::AppError; // Import the custom error type

// Handler for the /api/health endpoint
pub async fn health_check_handler() -> (StatusCode, Json<HealthStatus>) {
    // No input parameters or body to validate for this endpoint.
    // If there were, validation logic would go here.
    let health = HealthStatus {
        status: "healthy".to_string(),
    };
    (StatusCode::OK, Json(health))
}

// Handler for the /api/v1/quote endpoint
// Updated to return Result<Json<QuoteResponse>, AppError>
// Axum will automatically call `into_response()` on AppError if this handler returns Err.
pub async fn get_quote_handler() -> Result<Json<QuoteResponse>, AppError> {
    // TODO: Make the quotes file path configurable
    match quote_service::load_quotes_from_file("data/quotes.json") {
        Ok(quotes) => {
            if let Some(random_quote) = quote_service::get_random_quote(&quotes) {
                let response = QuoteResponse {
                    quote: random_quote.text.clone(),
                    author: random_quote.author.clone(),
                };
                Ok(Json(response))
            } else {
                // Use AppError for no quotes available
                Err(AppError::NotFound(
                    "No quotes available in the data file.".to_string(),
                ))
            }
        }
        Err(e) => {
            tracing::error!("Failed to load quotes: {}", e);
            // Convert the underlying error (e.g., std::io::Error) into AppError
            // The From<std::io::Error> for AppError in errors.rs will handle this if e is io::Error
            // If e is a different error type, a specific From impl or manual conversion is needed.
            // Assuming load_quotes_from_file returns an error that can be converted or is an io::Error.
            // For now, let's explicitly map it to QuoteSourcingError.
            Err(AppError::QuoteSourcingError(format!(
                "Failed to load quotes: {}",
                e
            )))
        }
    }
}