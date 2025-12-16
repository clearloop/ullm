//! The LLM implementation

use crate::DeepSeek;
use anyhow::Result;
use ucore::Client;

impl DeepSeek {
    /// Create a new LLM provider
    pub fn new(client: Client, key: &str) -> Result<Self> {
        Ok(Self {
            client,
            key: key.to_string(),
        })
    }
}
