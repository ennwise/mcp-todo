use axum::{
    body::{self, Body}, // Import axum::body directly
    http::{Request, StatusCode},
};
use rustquote_service::models::Quote;
use rustquote_service::responses::QuoteResponse;
use rustquote_service::app; // Use the app router from the library
use rustquote_service::AppState; // Import AppState
use std::sync::Arc; // For AppState
use serde_json;
// std::fs::File and std::io::Write are not directly needed in tests anymore if create_temp_quotes_file handles it
use tempfile::NamedTempFile;
// std::path::{Path, PathBuf} are not directly needed if NamedTempFile handles paths
use tower::util::ServiceExt; // Corrected import for ServiceExt

// Helper function to create a temporary quotes file and return it
// NamedTempFile handles its own deletion when it goes out of scope.
fn create_temp_quotes_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    std::io::Write::write_all(&mut file, content.as_bytes()).expect("Failed to write to temp file");
    file
}

#[tokio::test]
async fn test_get_quote_handler_success() {
    let quotes_content = r#"[
        {"id": 1, "quote": "This is a test quote.", "author": "Tester", "source": "Test Suite"},
        {"id": 2, "quote": "Another test quote.", "author": "Tester2", "source": null}
    ]"#;
    let temp_file = create_temp_quotes_file(quotes_content);
    let app_state = AppState {
        quotes_file_path: Arc::new(temp_file.path().to_str().unwrap().to_string()),
    };
    let router = app(app_state);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let quote_response: QuoteResponse = serde_json::from_slice(&body).expect("Failed to deserialize quote response");

    let original_quotes: Vec<Quote> = serde_json::from_str(quotes_content).unwrap();
    let is_valid_quote = original_quotes.iter().any(|q| q.text == quote_response.quote && q.author == quote_response.author);
    assert!(is_valid_quote, "Returned quote was not one of the test quotes.");
    // temp_file is automatically cleaned up
}

#[tokio::test]
async fn test_get_quote_handler_empty_file() {
    let temp_file = create_temp_quotes_file("[]"); // Empty JSON array
    let app_state = AppState {
        quotes_file_path: Arc::new(temp_file.path().to_str().unwrap().to_string()),
    };
    let router = app(app_state);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND); // Expect 404 if no quotes
    let body = body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_str = String::from_utf8_lossy(&body);
    println!("[test_get_quote_handler_empty_file] Response body: {}", body_str);
    let error_response: serde_json::Value = serde_json::from_slice(&body)
        .expect("[test_get_quote_handler_empty_file] Failed to deserialize error response");
    assert_eq!(error_response["message"], "No quotes available in the data file.");
    assert_eq!(error_response["error_code"], "NOT_FOUND");
    // temp_file is automatically cleaned up
}

#[tokio::test]
async fn test_get_quote_handler_file_not_found() {
    // Create a path that is guaranteed not to exist for a regular file
    let temp_file = NamedTempFile::new().unwrap();
    let non_existent_path = temp_file.path().to_str().unwrap().to_string();
    drop(temp_file); // Delete the temp file, ensuring path does not exist

    let app_state = AppState {
        quotes_file_path: Arc::new(non_existent_path),
    };
    let router = app(app_state);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body_str = String::from_utf8_lossy(&body);
    println!("[test_get_quote_handler_file_not_found] Response body: {}", body_str);
    let error_response: serde_json::Value = serde_json::from_slice(&body)
        .expect("[test_get_quote_handler_file_not_found] Failed to deserialize error response");
    let err_msg = error_response["message"].as_str().expect("Error message should be a string");
    // Message comes from AppError::from(QuoteServiceError::FileNotFound)
    assert!(err_msg.starts_with("Quote data file not found:"));
    assert_eq!(error_response["error_code"], "QUOTE_SOURCING_ERROR");
}

#[tokio::test]
async fn test_get_quote_handler_invalid_json() {
    let temp_file = create_temp_quotes_file("[{{\"id\":1, \"quote\":\"bad json\"}}]"); // Malformed JSON
    let app_state = AppState {
        quotes_file_path: Arc::new(temp_file.path().to_str().unwrap().to_string()),
    };
    let router = app(app_state);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let err_msg = error_response["message"].as_str().expect("Error message should be a string");
    println!("[test_get_quote_handler_invalid_json] Actual error message: {}", err_msg);
    assert!(err_msg.starts_with("Error parsing quote data:"));
    assert_eq!(error_response["error_code"], "QUOTE_SOURCING_ERROR");
    // temp_file is automatically cleaned up
}

#[tokio::test]
async fn test_get_quote_handler_malformed_quote_data() {
    let malformed_quotes_content = r#"[
        {"id": 1, "quote": "This is a valid quote.", "author": "Valid Author", "source": "Test Suite"},
        {"id": 2, "author": "Malformed Author"}
    ]"#; // Second quote is missing "quote"
    let temp_file = create_temp_quotes_file(malformed_quotes_content);
    let app_state = AppState {
        quotes_file_path: Arc::new(temp_file.path().to_str().unwrap().to_string()),
    };
    let router = app(app_state);

    let response = router
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let err_msg = error_response["message"].as_str().expect("Error message should be a string");
    println!("[test_get_quote_handler_malformed_quote_data] Actual error message: {}", err_msg);
    assert!(err_msg.starts_with("Error parsing quote data:"));
    assert!(err_msg.contains("missing field `quote`"));
    assert_eq!(error_response["error_code"], "QUOTE_SOURCING_ERROR");
    // temp_file is automatically cleaned up
    // std::fs::remove_file(&quote_file_path).expect("[test_get_quote_handler_malformed_quote_data] Failed to remove test quotes.json");
    // if data_dir.exists() && data_dir.read_dir().map_or(false, |mut d| d.next().is_none()) { std::fs::remove_dir(data_dir).ok(); }
}

#[tokio::test]
async fn test_health_check_handler() {
    // Health check doesn't use quotes_file_path, so a dummy one is fine.
    let dummy_app_state = AppState {
        quotes_file_path: Arc::new("dummy_path_for_health_check.json".to_string()),
    };
    let router = app(dummy_app_state);

    let response = router
        .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let health_status: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(health_status["status"], "healthy");
}