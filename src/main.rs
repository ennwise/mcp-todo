//! # RustQuote Service Binary
//!
//! This crate is the main entry point for the RustQuote service.
//! It initializes and runs the server defined in the `rustquote_service` library.

// The `rustquote_service` crate name comes from Cargo.toml's [package] name
use rustquote_service::run_server;

/// The main entry point for the RustQuote service.
///
/// Initializes the tracing subscriber for logging and calls the `run_server`
/// function from the `rustquote_service` library.
#[tokio::main]
async fn main() {
    // Initialize tracing
    // This is the appropriate place for the binary to initialize logging.
    tracing_subscriber::fmt::init();

    tracing::info!("Binary: Starting server...");
    let default_quotes_path = "data/quotes.json".to_string();
    if let Err(e) = run_server(default_quotes_path).await {
        tracing::error!("Binary: Server error: {}", e);
        std::process::exit(1);
    }
    tracing::info!("Binary: Server shut down gracefully.");
}
