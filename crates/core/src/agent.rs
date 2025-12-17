//! Turbofish Agent library

use crate::{StreamChunk, Tool, ToolCall, ToolChoice, message::ToolMessage};
use anyhow::Result;

/// A trait for turbofish agents
///
/// TODO: add schemar for request and response
pub trait Agent: Clone {
    /// The parsed chunk from [StreamChunk]
    type Chunk;

    /// The system prompt for the agent
    const SYSTEM_PROMPT: &str;

    /// The tools for the agent
    const TOOLS: Vec<Tool> = Vec::new();

    /// Filter the messages to match required tools for the agent
    fn filter(&self, _message: &str) -> ToolChoice {
        ToolChoice::Auto
    }

    /// Dispatch a tool call
    fn dispatch(&self, _tool: &[ToolCall]) -> impl Future<Output = Result<Vec<ToolMessage>>> {
        async move { anyhow::bail!("no tools available") }
    }

    /// Parse a chunk from [StreamChunk]
    fn chunk(&self, chunk: &StreamChunk) -> impl Future<Output = Result<Self::Chunk>>;
}

impl Agent for () {
    type Chunk = StreamChunk;

    const SYSTEM_PROMPT: &str = "You are a helpful assistant.";

    async fn chunk(&self, chunk: &StreamChunk) -> Result<Self::Chunk> {
        Ok(chunk.clone())
    }
}
