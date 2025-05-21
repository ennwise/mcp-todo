use serde::Serialize;

// Standardized Success Response for /api/health
#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
}

// Standardized Success Response for /api/v1/quote
#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: String,
    pub author: String,
}

// Standardized Error Response
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}