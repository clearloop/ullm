//! The request body for the DeepSeek API

use serde::Serialize;
use ucore::{ChatMessage, Config};

/// The request body for the DeepSeek API
#[derive(Debug, Clone, Serialize)]
pub struct Request<'r> {
    /// The configuration for the request
    #[serde(flatten)]
    pub config: &'r Config,

    /// The messages to send to the API
    pub messages: &'r [ChatMessage],

    /// Whether to stream the response
    pub stream: bool,
}

impl<'r> Request<'r> {
    /// Create a new request
    pub fn new(config: &'r Config, messages: &'r [ChatMessage]) -> Self {
        Self {
            config,
            messages,
            stream: false,
        }
    }

    /// Create a new request
    pub fn stream(config: &'r Config, messages: &'r [ChatMessage]) -> Self {
        Self {
            config,
            messages,
            stream: true,
        }
    }
}
