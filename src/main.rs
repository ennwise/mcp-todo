//! # RustQuote Service
//!
//! This crate provides a web service for fetching quotes.
//! It uses Axum for the web framework and Tokio as the asynchronous runtime.

mod api_handler;
mod responses;
mod config_manager;
mod models;
mod services;
mod quote_generator;
mod utils;
mod errors;

use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

/// The main entry point for the RustQuote service.
///
/// Initializes the tracing subscriber for logging, sets up the Axum router
/// with API endpoints, and starts the HTTP server.
#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = Router::new()
        .route("/api/health", get(api_handler::health_check_handler))
        .route("/api/v1/quote", get(api_handler::get_quote_handler));

    // Define the address to listen on
    // TODO: Make this configurable (e.g., from environment variable or config file)
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080)); // Changed port to 8080
    tracing::info!("listening on {}", addr);

    // Run the server
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();

    // The old main function content is removed as it's replaced by the server.
    // The quote loading and demonstration logic will be moved to appropriate API handlers later.
    /*
    use crate::services::quote_service;

    println!("Hello, world!");
    api_handler::placeholder_api_call();

    // Example of loading quotes
    println!("\nLoading quotes from file...");
    match quote_service::load_quotes_from_file("data/quotes.json") {
        Ok(quotes) => {
            if quotes.is_empty() {
                println!("No quotes found in the file.");
            } else {
                println!("Successfully loaded {} quotes:", quotes.len());
                for quote in quotes.iter().take(2) { // Print first 2 quotes as an example
                    println!("- \"{}\" - {} (Source: {:?})", quote.text, quote.author, quote.source.as_deref().unwrap_or("N/A"));
                }
                if quotes.len() > 2 {
                    println!("...and {} more.", quotes.len() - 2);
                }

                // Demonstrate get_random_quote
                println!("\nFetching a random quote...");
                if let Some(random_quote) = quote_service::get_random_quote(&quotes) {
                    println!("Random quote: \"{}\" - {} (Source: {:?})", random_quote.text, random_quote.author, random_quote.source.as_deref().unwrap_or("N/A"));
                } else {
                    println!("Could not get a random quote (maybe the list is empty?).");
                }

                // Demonstrate get_quote_by_id
                let id_to_find: u32 = 1; // Example ID
                println!("\nFetching quote by ID: {}...", id_to_find);
                if let Some(quote_by_id) = quote_service::get_quote_by_id(&quotes, id_to_find) {
                    println!("Quote by ID {}: \"{}\" - {} (Source: {:?})", id_to_find, quote_by_id.text, quote_by_id.author, quote_by_id.source.as_deref().unwrap_or("N/A"));
                } else {
                    println!("Quote with ID {} not found.", id_to_find);
                }

                // Demonstrate get_quote_by_id for a non-existent ID
                let non_existent_id: u32 = 999; // Example non-existent ID
                println!("\nFetching quote by ID: {} (expected not found)...", non_existent_id);
                if let Some(quote_by_id) = quote_service::get_quote_by_id(&quotes, non_existent_id) {
                    println!("Quote by ID {}: \"{}\" - {} (Source: {:?})", non_existent_id, quote_by_id.text, quote_by_id.author, quote_by_id.source.as_deref().unwrap_or("N/A"));
                } else {
                    println!("Quote with ID {} not found, as expected.", non_existent_id);
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading quotes: {}", e);
        }
    }
    */
}
