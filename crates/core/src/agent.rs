//! Turbofish Agent library

use crate::Message;
use serde::{Serialize, de::DeserializeOwned};

/// A trait for turbofish agents
///
/// TODO: add schemar for request and response
pub trait Agent {
    /// The system prompt for the agent
    const SYSTEM_PROMPT: &str;

    /// The request type for the agent
    type Request: Serialize + DeserializeOwned;

    /// The response type for the agent
    type Response: Serialize + DeserializeOwned;

    /// Get the system prompt for the agent
    fn system(embed: bool) -> Message {
        if !embed {
            return Message::system(Self::SYSTEM_PROMPT);
        }

        Message::system(format!(
            r#"{}
                
                EXAMPLE INPUT:
                {}
                
                EXAMPLE JSON OUTPUT:
                {}"#,
            Self::SYSTEM_PROMPT,
            serde_json::to_string(&Self::template_request()).unwrap(),
            serde_json::to_string(&Self::template_response()).unwrap()
        ))
    }

    /// Get the template for the request
    fn template_request() -> Self::Request;

    /// Get the template for the response
    fn template_response() -> Self::Response;
}
