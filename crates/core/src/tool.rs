//! Tool abstractions for the unified LLM Interfaces

use schemars::Schema;
use serde::{Deserialize, Serialize};

/// A tool for the LLM
#[derive(Debug, Clone, Serialize)]
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

/// Controls which tool is called by the model
#[derive(Debug, Clone)]
pub enum ToolChoice {
    /// Model will not call any tool
    None,

    /// Model can pick between generating a message or calling tools
    Auto,

    /// Model must call one or more tools
    Required,

    /// Model must call the specified function
    Function(ToolChoiceFunction),
}

/// A specific function to call
#[derive(Debug, Clone, Serialize)]
pub struct ToolChoiceFunction {
    /// The name of the function to call
    pub name: String,
}

impl ToolChoice {
    /// Create a tool choice for a specific function
    pub fn function(name: impl Into<String>) -> Self {
        ToolChoice::Function(ToolChoiceFunction { name: name.into() })
    }
}

impl Serialize for ToolChoice {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ToolChoice::None => serializer.serialize_str("none"),
            ToolChoice::Auto => serializer.serialize_str("auto"),
            ToolChoice::Required => serializer.serialize_str("required"),
            ToolChoice::Function(function) => {
                use serde::ser::SerializeStruct;
                let mut s = serializer.serialize_struct("ToolChoice", 2)?;
                s.serialize_field("type", "function")?;
                s.serialize_field("function", function)?;
                s.end()
            }
        }
    }
}
