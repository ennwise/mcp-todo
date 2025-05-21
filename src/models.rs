use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub id: u32,
    pub text: String,
    pub author: String,
    pub source: Option<String>,
}