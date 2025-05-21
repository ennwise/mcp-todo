use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use rust_quote_service::api_handler::{get_quote_handler, health_check_handler}; // Assuming your crate name is rust_quote_service
use rust_quote_service::models::Quote;
use rust_quote_service::responses::QuoteResponse;
use serde_json;
use std::fs::File;
use std::io::Write;
use tempfile::NamedTempFile;
use tower::ServiceExt; // for `oneshot`

// Helper function to create a temporary quotes file
fn create_temp_quotes_file(content: &str) -> NamedTempFile {
    let mut file = NamedTempFile::new().expect("Failed to create temp file");
    writeln!(file, "{}", content).expect("Failed to write to temp file");
    file
}

// Helper function to setup the app router for testing
fn app() -> Router {
    Router::new()
        .route("/api/v1/quote", axum::routing::get(get_quote_handler))
        .route("/api/health", axum::routing::get(health_check_handler))
}

#[tokio::test]
async fn test_get_quote_handler_success() {
    let quotes_content = r#"[
        {"id": 1, "text": "This is a test quote.", "author": "Tester", "source": "Test Suite"},
        {"id": 2, "text": "Another test quote.", "author": "Tester2", "source": null}
    ]"#;
    let temp_file = create_temp_quotes_file(quotes_content);
    let temp_file_path = temp_file.path().to_str().expect("Path to_str failed");

    // Override the default path in get_quote_handler for this test.
    // This is tricky because the handler itself hardcodes the path.
    // For a real application, the path should be configurable, e.g., via AppState.
    // For this test, we'll assume the handler can be modified or we test its behavior
    // by placing the temp file at "data/quotes.json" if the test environment allows.
    // As a workaround, we'd ideally refactor get_quote_handler to accept a path or a QuoteService instance.

    // Since we can't easily change the hardcoded path "data/quotes.json" in the handler
    // without refactoring, this test will effectively try to use that path.
    // To make this test pass in isolation without refactoring the handler,
    // we would need to ensure "data/quotes.json" contains the `quotes_content`.
    // This is not ideal for unit/integration testing as it creates external dependencies.

    // For the purpose of this exercise, let's simulate the ideal scenario where the handler
    // *could* use a path we provide. If the actual `get_quote_handler` is not changed
    // to accept a path, this test will fail unless "data/quotes.json" matches `quotes_content`.

    // A better approach for testing would be to refactor `get_quote_handler` to take `QuoteService`
    // as a dependency, or the file path as a parameter or from `State`.
    // Given the current structure of `get_quote_handler`, we will proceed by creating the
    // "data/quotes.json" file for the test.

    std::fs::create_dir_all("data").expect("Failed to create data directory");
    let mut test_quotes_file = File::create("data/quotes.json").expect("Failed to create test quotes.json");
    writeln!(test_quotes_file, "{}", quotes_content).expect("Failed to write to test quotes.json");


    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let quote_response: QuoteResponse = serde_json::from_slice(&body).expect("Failed to deserialize quote response");

    let original_quotes: Vec<Quote> = serde_json::from_str(quotes_content).unwrap();
    let is_valid_quote = original_quotes.iter().any(|q| q.text == quote_response.quote && q.author == quote_response.author);
    assert!(is_valid_quote, "Returned quote was not one of the test quotes.");

    // Clean up the created file
    std::fs::remove_file("data/quotes.json").expect("Failed to remove test quotes.json");
    std::fs::remove_dir("data").ok(); // Remove dir if empty, ok if not (e.g. if it existed before)
}

#[tokio::test]
async fn test_get_quote_handler_empty_file() {
    // Create an empty quotes file
    std::fs::create_dir_all("data").expect("Failed to create data directory");
    let mut file = File::create("data/quotes.json").expect("Failed to create empty quotes.json");
    writeln!(file, "[]").expect("Failed to write empty array to quotes.json");

    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(error_response["error"], "No quotes available in the data file.");

    std::fs::remove_file("data/quotes.json").expect("Failed to remove test quotes.json");
    std::fs::remove_dir("data").ok();
}

#[tokio::test]
async fn test_get_quote_handler_file_not_found() {
    // Ensure the quotes file does not exist
    std::fs::remove_file("data/quotes.json").ok(); // .ok() to ignore error if not found
    std::fs::remove_dir("data").ok();


    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR); // Or specific error code from AppError
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    // The exact message depends on how QuoteServiceError::FileNotFound is mapped to AppError
    // In api_handler, it becomes: AppError::QuoteSourcingError(format!("Failed to load quotes: {}", e))
    // where e is QuoteServiceError::FileNotFound.
    assert!(error_response["error"].as_str().unwrap().contains("Failed to load quotes"));
    assert!(error_response["error"].as_str().unwrap().contains("Quote data file not found at path: data/quotes.json"));
}

#[tokio::test]
async fn test_get_quote_handler_invalid_json() {
    std::fs::create_dir_all("data").expect("Failed to create data directory");
    let mut file = File::create("data/quotes.json").expect("Failed to create invalid quotes.json");
    writeln!(file, "[{{\"id\":1, \"text\":\"bad json\"}}]").expect("Failed to write invalid json"); // Malformed

    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(error_response["error"].as_str().unwrap().contains("Failed to load quotes"));
    assert!(error_response["error"].as_str().unwrap().contains("Error parsing quote data"));


    std::fs::remove_file("data/quotes.json").expect("Failed to remove test quotes.json");
    std::fs::remove_dir("data").ok();
}

#[tokio::test]
async fn test_get_quote_handler_malformed_quote_data() {
    // Test with a quotes file that has a valid JSON array but a malformed Quote object inside
    // (e.g., missing a required field like "text")
    std::fs::create_dir_all("data").expect("Failed to create data directory");
    let malformed_quotes_content = r#"[
        {"id": 1, "text": "This is a valid quote.", "author": "Valid Author", "source": "Test Suite"},
        {"id": 2, "author": "Malformed Author"}
    ]"#; // Second quote is missing "text"
    let mut file = File::create("data/quotes.json").expect("Failed to create malformed quotes.json");
    writeln!(file, "{}", malformed_quotes_content).expect("Failed to write malformed json");

    let app = app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/quote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let error_response: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(error_response["error"].as_str().unwrap().contains("Failed to load quotes"));
    // The specific error from serde for missing field might be "missing field `text`" or similar
    assert!(error_response["error"].as_str().unwrap().contains("Error parsing quote data"));


    std::fs::remove_file("data/quotes.json").expect("Failed to remove test quotes.json");
    std::fs::remove_dir("data").ok();
}

#[tokio::test]
async fn test_health_check_handler() {
    let app = app();

    let response = app
        .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let health_status: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(health_status["status"], "healthy");
}