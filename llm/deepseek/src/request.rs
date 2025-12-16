//! The request body for the DeepSeek API

use serde::Serialize;
use serde_json::{Number, Value, json};
use ucore::{ChatMessage, Config};

/// The request body for the DeepSeek API
#[derive(Debug, Clone, Serialize)]
pub struct Request<'r> {
    /// Whether to return the log probabilities
    pub logprobs: Value,

    /// The maximum number of tokens to generate
    pub max_tokens: usize,

    /// The messages to send to the API
    pub messages: &'r [ChatMessage],

    /// The model we are using
    pub model: &'static str,

    /// The response format to use
    pub response_format: Value,

    /// Whether to stream the response
    pub stream: bool,

    /// Whether to enable thinking
    pub thinking: Value,

    /// The temperature to use for the response
    pub temperature: Value,

    /// An integer between 0 and 20 specifying the number of most likely tokens to
    /// return at each token position, each with an associated log probability.
    pub top_logprobs: Value,
}

impl<'r> Request<'r> {
    /// Create a new request
    pub fn new(config: &'r Config, messages: &'r [ChatMessage], stream: bool) -> Self {
        Self {
            logprobs: Value::Bool(config.logprobs),
            max_tokens: config.tokens,
            messages,
            model: config.model,
            response_format: if config.json {
                json!({ "type": "json_object" })
            } else {
                Value::Null
            },
            stream,
            temperature: if let Some(temperature) = Number::from_f64(config.temperature as f64) {
                Value::Number(temperature)
            } else {
                json!(1.0)
            },
            thinking: if config.think {
                json!({ "type": "enabled" })
            } else {
                Value::Null
            },
            top_logprobs: if config.logprobs {
                Value::Number(config.top_logprobs.into())
            } else {
                Value::Null
            },
        }
    }
}
