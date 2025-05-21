use serde::{Serialize, Deserialize};

// WBS 2.4: Implement Quote Formatting Logic
// For the MVP, the Quote struct is directly serialized for API responses.
// The `text` field is renamed to `quote` to match the API specification.
// Extra fields like `id` and `source` may be present in the response but are acceptable.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Quote {
    pub id: u32,
    #[serde(rename = "quote")]
    pub text: String,
    pub author: String,
    pub source: Option<String>,
}

impl Quote {
    // A constructor might be useful later
    pub fn new(id: u32, text: String, author: String, source: Option<String>) -> Self {
        Quote {
            id,
            text,
            author,
            source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_serialization() {
        let quote = Quote {
            id: 1,
            text: "This is a test quote.".to_string(),
            author: "Test Author".to_string(),
            source: Some("Test Source".to_string()),
        };
        let serialized = serde_json::to_string(&quote).unwrap();
        let expected_json = r#"{"id":1,"quote":"This is a test quote.","author":"Test Author","source":"Test Source"}"#;
        assert_eq!(serialized, expected_json);
    }

    #[test]
    fn test_quote_deserialization() {
        let json_data = r#"{"id":2,"quote":"Another test.","author":"Another Author","source":null}"#;
        let deserialized: Quote = serde_json::from_str(json_data).unwrap();
        let expected_quote = Quote {
            id: 2,
            text: "Another test.".to_string(),
            author: "Another Author".to_string(),
            source: None,
        };
        assert_eq!(deserialized, expected_quote);
    }

    #[test]
    fn test_quote_deserialization_with_source() {
        let json_data = r#"{"id":3,"quote":"With source.","author":"Source Author","source":"The Source"}"#;
        let deserialized: Quote = serde_json::from_str(json_data).unwrap();
        let expected_quote = Quote {
            id: 3,
            text: "With source.".to_string(),
            author: "Source Author".to_string(),
            source: Some("The Source".to_string()),
        };
        assert_eq!(deserialized, expected_quote);
    }

    #[test]
    fn test_new_quote_constructor() {
        let quote = Quote::new(4, "Constructed quote".to_string(), "Constructor".to_string(), Some("Source of Construction".to_string()));
        assert_eq!(quote.id, 4);
        assert_eq!(quote.text, "Constructed quote");
        assert_eq!(quote.author, "Constructor");
        assert_eq!(quote.source, Some("Source of Construction".to_string()));
    }
}