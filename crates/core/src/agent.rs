//! Turbofish Agent library

use crate::{Tool, ToolCall, ToolChoice, message::ToolMessage};
use anyhow::Result;

/// A trait for turbofish agents
///
/// TODO: add schemar for request and response
pub trait Agent {
    /// The system prompt for the agent
    const SYSTEM_PROMPT: &str;

    /// The tools for the agent
    const TOOLS: Vec<Tool> = Vec::new();

    /// Filter the messages to match required tools for the agent
    fn filter(&self, _message: &str) -> ToolChoice {
        ToolChoice::Auto
    }

    /// Dispatch a tool call
    fn dispatch(&self, _tool: &ToolCall) -> impl Future<Output = Result<ToolMessage>> {
        async move { anyhow::bail!("no tools available") }
    }
}
