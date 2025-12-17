//! Chat abstractions for the unified LLM Interfaces

use crate::{
    Agent, Config, General, LLM, Response, Role,
    message::{AssistantMessage, Message, ToolMessage},
};
use anyhow::Result;
use futures_core::Stream;
use futures_util::StreamExt;
use serde::Serialize;

/// A chat for the LLM
#[derive(Clone)]
pub struct Chat<P: LLM, A: Agent> {
    /// The chat configuration
    pub config: P::ChatConfig,

    /// Chat history in memory
    pub messages: Vec<ChatMessage>,

    /// The LLM provider
    provider: P,

    /// The agent
    agent: A,

    /// Whether to return the usage information in stream mode
    usage: bool,
}

impl<P: LLM> Chat<P, ()> {
    /// Create a new chat
    pub fn new(config: General, provider: P) -> Self {
        Self {
            messages: vec![],
            provider,
            usage: config.usage,
            agent: (),
            config: config.into(),
        }
    }
}

impl<P: LLM, A: Agent> Chat<P, A> {
    /// Add the system prompt to the chat
    pub fn system<B: Agent>(mut self, agent: B) -> Chat<P, B> {
        let mut messages = self.messages;
        if messages.is_empty() {
            messages.push(Message::system(A::SYSTEM_PROMPT).into());
        } else if let Some(ChatMessage::System(_)) = messages.first() {
            messages.insert(0, Message::system(A::SYSTEM_PROMPT).into());
        } else {
            messages = vec![Message::system(A::SYSTEM_PROMPT).into()]
                .into_iter()
                .chain(messages)
                .collect();
        }

        self.config = self.config.with_tools(A::TOOLS);
        Chat {
            messages,
            provider: self.provider,
            usage: self.usage,
            agent,
            config: self.config,
        }
    }

    /// Send a message to the LLM
    pub async fn send(&mut self, message: Message) -> Result<Response> {
        let config = self
            .config
            .with_tool_choice(self.agent.filter(message.content.as_str()));
        self.messages.push(message.into());
        self.provider.send(&config, &self.messages).await
    }

    /// Send a message to the LLM with streaming
    pub fn stream(
        &mut self,
        message: Message,
    ) -> impl Stream<Item = Result<A::Chunk>> + use<'_, P, A> {
        let config = self
            .config
            .with_tool_choice(self.agent.filter(message.content.as_str()));
        self.messages.push(message.into());
        let agent = self.agent.clone();
        self.provider
            .stream(config, &self.messages, self.usage)
            .then(move |chunk| {
                let agent = agent.clone();
                async move {
                    match chunk {
                        Ok(c) => agent.chunk(&c).await,
                        Err(e) => Err(e),
                    }
                }
            })
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
        match message.role {
            Role::User => ChatMessage::User(message),
            Role::Assistant => ChatMessage::Assistant(AssistantMessage {
                message,
                prefix: false,
                reasoning: String::new(),
            }),
            Role::System => ChatMessage::System(message),
            Role::Tool => ChatMessage::Tool(ToolMessage {
                tool: String::new(),
                message,
            }),
        }
    }
}
