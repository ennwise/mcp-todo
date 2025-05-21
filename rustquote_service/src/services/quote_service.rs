//! # Quote Service
//!
//! This module provides services related to managing and retrieving quotes.
//! It includes functionality for loading quotes from a data source,
//! and fetching specific or random quotes.

use crate::models::quote::Quote;
use serde_json;
use std::fs;
use std::io;
use std::path::Path;

// Define a custom error type for quote loading issues
#[derive(Debug)]
pub enum QuoteServiceError {
    FileNotFound(String),
    FileReadError(io::Error),
    ParseError(serde_json::Error),
}

impl std::fmt::Display for QuoteServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QuoteServiceError::FileNotFound(path) => write!(f, "Quote data file not found at path: {}", path),
            QuoteServiceError::FileReadError(err) => write!(f, "Error reading quote data file: {}", err),
            QuoteServiceError::ParseError(err) => write!(f, "Error parsing quote data: {}", err),
        }
    }
}

/// Implements the `std::error::Error` trait for [`QuoteServiceError`].
///
/// This allows [`QuoteServiceError`] to be used as a standard Rust error type,
/// enabling features like error chaining.
impl std::error::Error for QuoteServiceError {
    /// Returns the underlying cause of the error, if any.
    ///
    /// For `FileReadError` and `ParseError` variants, this will return the
    /// wrapped `io::Error` or `serde_json::Error` respectively.
    /// For `FileNotFound`, it returns `None`.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            QuoteServiceError::FileReadError(err) => Some(err),
            QuoteServiceError::ParseError(err) => Some(err),
            _ => None,
        }
    }
}

/// Loads quotes from a JSON file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the quotes JSON file.
///
/// # Errors
///
/// Returns `QuoteServiceError` if the file cannot be found, read, or parsed.
pub fn load_quotes_from_file(file_path_str: &str) -> Result<Vec<Quote>, QuoteServiceError> {
    let path = Path::new(file_path_str);

    if !path.exists() {
        return Err(QuoteServiceError::FileNotFound(file_path_str.to_string()));
    }

    let file_content = fs::read_to_string(path)
        .map_err(QuoteServiceError::FileReadError)?;

    let quotes: Vec<Quote> = serde_json::from_str(&file_content)
        .map_err(QuoteServiceError::ParseError)?;

    Ok(quotes)
}

/// Returns a random quote from a slice of [`Quote`]s.
///
/// # Arguments
///
/// * `quotes` - A slice of [`Quote`] structs from which to select a random quote.
///
/// # Returns
///
/// An `Option` containing a reference to a random [`Quote`] if the slice is not empty,
/// otherwise `None`.
pub fn get_random_quote(quotes: &[Quote]) -> Option<&Quote> {
    if quotes.is_empty() {
        None
    } else {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        quotes.choose(&mut rng)
    }
}

/// Finds and returns a reference to a quote by its ID from a slice of [`Quote`]s.
///
/// # Arguments
///
/// * `quotes` - A slice of [`Quote`] structs to search within.
/// * `id` - The ID of the quote to find.
///
/// # Returns
///
/// An `Option` containing a reference to the [`Quote`] if found, otherwise `None`.
pub fn get_quote_by_id(quotes: &[Quote], id: u32) -> Option<&Quote> {
    quotes.iter().find(|q| q.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_json_file(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", content).unwrap();
        file
    }

    #[test]
    fn test_load_quotes_successfully() {
        let sample_json = r#"[
            { "id": 1, "quote": "Test quote 1", "author": "Author 1", "source": "Source 1" },
            { "id": 2, "quote": "Test quote 2", "author": "Author 2", "source": null }
        ]"#;
        let temp_file = create_temp_json_file(sample_json);
        let quotes = load_quotes_from_file(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(quotes.len(), 2);
        assert_eq!(quotes[0].id, 1);
        assert_eq!(quotes[0].text, "Test quote 1");
        assert_eq!(quotes[1].author, "Author 2");
        assert_eq!(quotes[1].source, None);
    }

    #[test]
    fn test_load_quotes_file_not_found() {
        let result = load_quotes_from_file("non_existent_file.json");
        assert!(matches!(result, Err(QuoteServiceError::FileNotFound(_))));
    }

    #[test]
    fn test_load_quotes_invalid_json() {
        let invalid_json = r#"[
            { "id": 1, "text": "Test quote 1", "author": "Author 1", "source": "Source 1" },
            { "id": 2, "text": "Test quote 2", "author": "Author 2", "source": null },, // extra comma
        ]"#;
        let temp_file = create_temp_json_file(invalid_json);
        let result = load_quotes_from_file(temp_file.path().to_str().unwrap());
        assert!(matches!(result, Err(QuoteServiceError::ParseError(_))));
    }

    #[test]
    fn test_load_quotes_empty_file() {
        let empty_json = "";
        let temp_file = create_temp_json_file(empty_json);
        let result = load_quotes_from_file(temp_file.path().to_str().unwrap());
        assert!(matches!(result, Err(QuoteServiceError::ParseError(_))));
    }

    #[test]
    fn test_load_quotes_empty_array() {
        let empty_array_json = "[]";
        let temp_file = create_temp_json_file(empty_array_json);
        let quotes = load_quotes_from_file(temp_file.path().to_str().unwrap()).unwrap();
        assert_eq!(quotes.len(), 0);
    }

    #[test]
    fn test_get_random_quote_empty_list() {
        let quotes: Vec<Quote> = Vec::new();
        assert!(get_random_quote(&quotes).is_none());
    }

    #[test]
    fn test_get_random_quote_single_item() {
        let quotes = vec![
            Quote { id: 1, text: "Only quote".to_string(), author: "Author".to_string(), source: None },
        ];
        let random_quote = get_random_quote(&quotes);
        assert!(random_quote.is_some());
        assert_eq!(random_quote.unwrap().id, 1);
    }

    #[test]
    fn test_get_random_quote_multiple_items() {
        let quotes = vec![
            Quote { id: 1, text: "Quote 1".to_string(), author: "Author 1".to_string(), source: None },
            Quote { id: 2, text: "Quote 2".to_string(), author: "Author 2".to_string(), source: None },
            Quote { id: 3, text: "Quote 3".to_string(), author: "Author 3".to_string(), source: None },
        ];
        // Run multiple times to increase chance of catching non-random behavior (though not a perfect test for randomness)
        let mut found_ids = std::collections::HashSet::new();
        for _ in 0..100 {
            if let Some(quote) = get_random_quote(&quotes) {
                found_ids.insert(quote.id);
            } else {
                panic!("Expected a quote but got None");
            }
        }
        // Check if we got at least more than one unique quote, ideally all if the list is small
        assert!(found_ids.len() > 1 && found_ids.len() <= quotes.len());
    }

    #[test]
    fn test_get_quote_by_id_found() {
        let quotes = vec![
            Quote { id: 1, text: "Quote 1".to_string(), author: "Author 1".to_string(), source: None },
            Quote { id: 2, text: "Quote 2".to_string(), author: "Author 2".to_string(), source: Some("Source 2".to_string()) },
        ];
        let quote = get_quote_by_id(&quotes, 2);
        assert!(quote.is_some());
        assert_eq!(quote.unwrap().id, 2);
        assert_eq!(quote.unwrap().text, "Quote 2");
    }

    #[test]
    fn test_get_quote_by_id_not_found() {
        let quotes = vec![
            Quote { id: 1, text: "Quote 1".to_string(), author: "Author 1".to_string(), source: None },
        ];
        let quote = get_quote_by_id(&quotes, 99);
        assert!(quote.is_none());
    }

    #[test]
    fn test_get_quote_by_id_empty_list() {
        let quotes: Vec<Quote> = Vec::new();
        let quote = get_quote_by_id(&quotes, 1);
        assert!(quote.is_none());
    }
}