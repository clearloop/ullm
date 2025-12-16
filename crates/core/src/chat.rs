//! Chat abstractions for the unified LLM Interfaces

use crate::{
    LLM, Response, Role, StreamChunk,
    message::{AssistantMessage, Message, ToolMessage},
};
use anyhow::Result;
use futures_core::Stream;
use serde::Serialize;

/// A chat for the LLM
pub struct Chat<P: LLM> {
    /// The chat configuration
    pub config: P::ChatConfig,

    /// Chat history in memory
    pub messages: Vec<ChatMessage>,

    /// The LLM provider
    pub provider: P,
}

impl<P: LLM> Chat<P> {
    /// Send a message to the LLM
    pub async fn send(&mut self, message: Message) -> Result<Response> {
        self.messages.push(message.into());
        self.provider.send(&self.config, &self.messages).await
    }

    /// Send a message to the LLM with streaming
    pub fn stream(&mut self, message: Message) -> impl Stream<Item = Result<StreamChunk>> {
        self.messages.push(message.into());
        self.provider.stream(&self.config, &self.messages)
    }
}

/// A chat message in memory
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatMessage {
    /// A user message
    User(Message),

    /// An assistant message
    Assistant(AssistantMessage),

    /// A tool message
    Tool(ToolMessage),

    /// A system message
    System(Message),
}

impl From<Message> for ChatMessage {
    fn from(message: Message) -> Self {
        if message.role == Role::User {
            ChatMessage::User(message)
        } else {
            ChatMessage::System(message)
        }
    }
}
