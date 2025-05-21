//! # RustQuote Service Library
//!
//! This crate provides the core logic for the RustQuote web service.
//! It uses Axum for the web framework and Tokio as the asynchronous runtime.

// Module declarations
pub mod api_handler;
pub mod responses;
// pub mod config_manager; // This is now a top-level crate module, not part of rustquote_service library
pub mod errors;
pub mod models;
pub mod quote_generator;
pub mod services;
pub mod utils;

use axum::{routing::get, Router}; // Removed unused State import here, it's used in api_handler
use std::net::SocketAddr;
use std::path::PathBuf; // Added for quotes_file_path type
use std::sync::Arc; // For AppState
use tokio::net::TcpListener;

/// Application state shared across handlers.
#[derive(Clone)] // Must be Clone to be used as Axum state
pub struct AppState {
    pub quotes_file_path: Arc<PathBuf>, // Changed to PathBuf
}

/// Configures and runs the Axum web server.
///
/// Initializes the tracing subscriber for logging, sets up the Axum router
/// with API endpoints, and starts the HTTP server.
pub async fn run_server(
    server_address: SocketAddr,
    quotes_file_path: PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState {
        quotes_file_path: Arc::new(quotes_file_path),
    };

    // Build our application with routes
    let app = app(app_state); // Pass state to app()

    tracing::info!("Library: listening on {}", server_address);

    let listener = TcpListener::bind(server_address).await?;
    tracing::info!("Library: server bound to {}", server_address);
    axum::serve(listener, app.into_make_service()).await?;
    tracing::info!("Library: server finished");
    Ok(())
}

// Function to create the app router, useful for testing and main setup
// Now accepts AppState
pub fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/test", get(|| async {"Hello from test!"}))
        .route("/health", get(api_handler::health_check_handler))
        .route("/quote", get(api_handler::get_quote_handler))
        .route(
            "/api/v1/quote/:id",
            get(api_handler::get_quote_by_id_handler),
        )
        .with_state(app_state) // Share AppState with handlers
}
