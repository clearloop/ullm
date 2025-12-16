//! The LLM provider

use ucore::Client;

mod llm;

/// The DeepSeek LLM provider
pub struct DeepSeek {
    /// The HTTP client
    pub client: Client,

    /// The API key
    pub key: String,
}
