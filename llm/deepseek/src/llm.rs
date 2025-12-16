//! The LLM implementation

use crate::{DeepSeek, Request};
use anyhow::Result;
use ucore::{
    ChatMessage, Client, Config, LLM, Response,
    reqwest::{
        Method,
        header::{self, HeaderMap},
    },
};

const ENDPOINT: &str = "https://api.deepseek.com/chat/completions";

impl LLM for DeepSeek {
    /// Create a new LLM provider
    fn new(client: Client, key: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, "application/json".parse()?);
        headers.insert(header::ACCEPT, "application/json".parse()?);
        headers.insert(header::AUTHORIZATION, format!("Bearer {}", key).parse()?);
        Ok(Self { client, headers })
    }

    /// Send a message to the LLM
    async fn send(&mut self, config: &Config, messages: &[ChatMessage]) -> Result<Response> {
        self.client
            .request(Method::POST, ENDPOINT)
            .headers(self.headers.clone())
            .json(&Request::new(config, messages))
            .send()
            .await?
            .json::<Response>()
            .await
            .map_err(Into::into)
    }
}
