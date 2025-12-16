//! Configuration for a chat

use serde::Serialize;

/// Chat configuration
#[derive(Debug, Clone, Serialize)]
pub struct Config {
    /// The frequency penalty of the model
    pub frequency: i8,

    /// Whether to response in JSON
    pub json: bool,

    /// Whether to return the log probabilities
    pub logprobs: bool,

    /// The model to use
    pub model: &'static str,

    /// The presence penalty of the model
    pub presence: i8,

    /// The temperature of the model
    pub temperature: f32,

    /// Whether to enable thinking
    pub think: bool,

    /// The top probability of the model
    pub top_p: f32,

    /// The number of top log probabilities to return
    pub top_logprobs: usize,

    /// The number of max tokens to generate
    pub tokens: usize,

    /// Whether to return the usage information in stream mode
    pub usage: bool,
}

impl Config {
    /// Create a new configuration
    pub fn new(model: &'static str) -> Self {
        Self {
            model,
            ..Default::default()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frequency: 0,
            json: false,
            logprobs: false,
            model: "deepseek-chat",
            presence: 0,
            temperature: 1.0,
            think: false,
            top_logprobs: 0,
            top_p: 1.0,
            tokens: 1000,
            usage: true,
        }
    }
}
