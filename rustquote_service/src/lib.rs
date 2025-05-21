//! # RustQuote Service Library
//!
//! This crate provides the core logic for the RustQuote web service.
//! It uses Axum for the web framework and Tokio as the asynchronous runtime.

// Module declarations
pub mod api_handler;
pub mod responses;
pub mod config_manager;
pub mod models;
pub mod services;
pub mod quote_generator;
pub mod utils;
pub mod errors;

use axum::{routing::get, Router}; // Removed unused State import here, it's used in api_handler
use std::net::SocketAddr;
use std::sync::Arc; // For AppState
use tokio::net::TcpListener;
use tracing;

/// Application state shared across handlers.
#[derive(Clone)] // Must be Clone to be used as Axum state
pub struct AppState {
    pub quotes_file_path: Arc<String>, // Use Arc for shared ownership if needed, or just String
}

/// Configures and runs the Axum web server.
///
/// Initializes the tracing subscriber for logging, sets up the Axum router
/// with API endpoints, and starts the HTTP server.
pub async fn run_server(quotes_file_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState {
        quotes_file_path: Arc::new(quotes_file_path),
    };

    // Build our application with routes
    let app = app(app_state); // Pass state to app()

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Library: listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Library: server bound to {}", addr);
    axum::serve(listener, app.into_make_service()).await?;
    tracing::info!("Library: server finished");
    Ok(())
}

// Function to create the app router, useful for testing and main setup
// Now accepts AppState
pub fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(api_handler::health_check_handler))
        .route("/api/v1/quote", get(api_handler::get_quote_handler))
        .route("/api/v1/quote/:id", get(api_handler::get_quote_by_id_handler))
        .with_state(app_state) // Share AppState with handlers
}