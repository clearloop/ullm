//! Provider abstractions for the unified LLM Interfaces

use crate::Message;
use anyhow::Result;
use reqwest::Client;

/// A trait for LLM providers
pub trait LLM: Sized {
    /// Create a new LLM provider
    fn new(client: Client, key: &str) -> Result<Self>
    where
        Self: Sized;

    /// Send a message to the LLM
    fn send(&mut self, _message: Message) -> impl Future<Output = Result<String>>;
}
