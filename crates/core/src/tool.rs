//! Tool abstractions for the unified LLM Interfaces

use schemars::Schema;
use serde::{Deserialize, Serialize};

/// A tool for the LLM
pub struct Tool {
    /// The name of the tool
    pub name: &'static str,

    /// The description of the tool
    pub description: &'static str,

    /// The parameters of the tool
    pub parameters: Schema,

    /// Whether to strictly validate the parameters
    pub strict: bool,
}

/// A tool call made by the model
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolCall {
    /// The ID of the tool call
    pub id: String,

    /// The type of tool (currently only "function")
    #[serde(rename = "type")]
    pub call_type: String,

    /// The function to call
    pub function: FunctionCall,
}

/// A function call within a tool call
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionCall {
    /// The name of the function to call
    pub name: String,

    /// The arguments to pass to the function (JSON string)
    pub arguments: String,
}
