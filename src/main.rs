//! # RustQuote Service Binary
//!
//! This crate is the main entry point for the RustQuote service.
//! It initializes and runs the server defined in the `rustquote_service` library.

// The `rustquote_service` crate name comes from Cargo.toml's [package] name
use rustquote_service::run_server;
// Assuming config_manager.rs is in src/ alongside main.rs
mod config_manager;
use config_manager::load_config;

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

    // Load configuration
    let app_config = match load_config() {
        Ok(config) => {
            tracing::info!("Binary: Configuration loaded successfully: {:?}", config);
            config
        }
        Err(e) => {
            tracing::error!("Binary: Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = run_server(app_config.server_address, app_config.quotes_file_path).await {
        tracing::error!("Binary: Server error: {}", e);
        std::process::exit(1);
    }
    tracing::info!("Binary: Server shut down gracefully.");
}
