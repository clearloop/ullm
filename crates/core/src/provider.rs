//! Provider abstractions for the unified LLM Interfaces

use crate::{Chat, ChatMessage, Config, Response};
use anyhow::Result;
use reqwest::Client;

/// A trait for LLM providers
pub trait LLM: Sized + Clone {
    /// The chat configuration.
    type ChatConfig: From<Config>;

    /// Create a new LLM provider
    fn new(client: Client, key: &str) -> Result<Self>
    where
        Self: Sized;

    /// Create a new chat
    fn chat(&self, config: Config) -> Chat<Self> {
        Chat {
            config: config.into(),
            messages: Vec::new(),
            provider: self.clone(),
        }
    }

    /// Send a message to the LLM
    fn send(
        &mut self,
        config: &Self::ChatConfig,
        messages: &[ChatMessage],
    ) -> impl Future<Output = Result<Response>>;
}
