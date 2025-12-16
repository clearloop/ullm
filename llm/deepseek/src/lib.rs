//! The LLM provider

use ucore::Client;

mod llm;

/// The DeepSeek LLM provider
pub struct DeepSeek {
    /// The HTTP client
    client: Client,

    /// The API key
    key: String,
}
