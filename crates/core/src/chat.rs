//! Chat abstractions for the unified LLM Interfaces

use crate::{
    LLM,
    message::{AssistantMessage, Message, ToolMessage},
};

/// A chat for the LLM
pub struct Chat<P: LLM> {
    /// Chat history in memory
    pub messages: Vec<ChatMessage>,

    /// The LLM provider
    pub provider: P,
}

/// A chat message in memory
#[derive(Debug, Clone)]
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
