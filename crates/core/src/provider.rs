//! Provider abstractions for the unified LLM Interfaces

use crate::{Agent, Chat, ChatMessage, Config, Message, Response, StreamChunk};
use anyhow::Result;
use futures_core::Stream;
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
    fn chat<A: Agent>(&self, config: Config) -> Chat<Self> {
        Chat {
            config: config.into(),
            messages: vec![Message::system(A::SYSTEM_PROMPT).into()],
            provider: self.clone(),
        }
    }

    /// Send a message to the LLM
    fn send(
        &mut self,
        config: &Self::ChatConfig,
        messages: &[ChatMessage],
    ) -> impl Future<Output = Result<Response>>;

    /// Send a message to the LLM with streaming
    fn stream(
        &mut self,
        config: &Self::ChatConfig,
        messages: &[ChatMessage],
    ) -> impl Stream<Item = Result<StreamChunk>>;
}
