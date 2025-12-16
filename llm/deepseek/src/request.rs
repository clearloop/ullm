//! The request body for the DeepSeek API

use serde::Serialize;
use ucore::{ChatMessage, Config};

/// The request body for the DeepSeek API
#[derive(Debug, Clone, Serialize)]
pub struct Request<'r> {
    /// The messages to send to the API
    pub messages: &'r [ChatMessage],

    /// The configuration for the request
    #[serde(flatten)]
    pub config: &'r Config,
}

impl<'r> Request<'r> {
    /// Create a new request
    pub fn new(config: &'r Config, messages: &'r [ChatMessage]) -> Self {
        Self { messages, config }
    }
}
