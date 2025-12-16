//! Chat command

use crate::DeepSeek;
use anyhow::Result;
use clap::{Args, ValueEnum};
use futures_util::StreamExt;
use std::io::{BufRead, Write};
use ucore::{ChatMessage, Client, Config, LLM, Message};

/// Chat with an LLM
#[derive(Debug, Args)]
pub struct Chat {
    /// The model provider to use
    #[arg(short, long, default_value = "deepseek")]
    pub model: Model,

    /// The message to send (if empty, starts interactive mode)
    pub message: Option<String>,
}

impl Chat {
    /// Run the chat command
    pub async fn run(&self, stream_mode: bool) -> Result<()> {
        let key = std::env::var("DEEPSEEK_API_KEY")?;
        let mut provider = match self.model {
            Model::Deepseek => DeepSeek::new(Client::new(), &key)?,
        };

        let config = Config::default();
        let mut messages: Vec<ChatMessage> = Vec::new();

        if let Some(msg) = &self.message {
            messages.push(Message::user(msg).into());
            Self::send(&mut provider, &config, &mut messages, stream_mode).await?;
        } else {
            let stdin = std::io::stdin();
            let mut stdout = std::io::stdout();

            loop {
                print!("> ");
                stdout.flush()?;

                let mut input = String::new();
                if stdin.lock().read_line(&mut input)? == 0 {
                    break;
                }

                let input = input.trim();
                if input.is_empty() {
                    continue;
                }
                if input == "/quit" || input == "/exit" {
                    break;
                }

                messages.push(Message::user(input).into());
                Self::send(&mut provider, &config, &mut messages, stream_mode).await?;
            }
        }

        Ok(())
    }

    async fn send(
        provider: &mut DeepSeek,
        config: &Config,
        messages: &mut Vec<ChatMessage>,
        stream_mode: bool,
    ) -> Result<()> {
        if stream_mode {
            let mut response_content = String::new();
            {
                let mut reasoning = false;
                let mut stream = std::pin::pin!(provider.stream(config, messages));
                while let Some(chunk) = stream.next().await {
                    let chunk = chunk?;
                    if let Some(content) = chunk.content() {
                        if reasoning {
                            print!("\ncontent: ");
                            reasoning = false;
                        }
                        print!("{content}");
                        response_content.push_str(content);
                    }

                    if let Some(reasoning_content) = chunk.reasoning_content() {
                        if !reasoning {
                            print!("thinking: ");
                            reasoning = true;
                        }
                        print!("{reasoning_content}");
                        response_content.push_str(reasoning_content);
                    }
                }
            }
            println!();
            messages.push(Message::assistant(&response_content).into());
        } else {
            let response = provider.send(config, messages).await?;
            if let Some(reasoning_content) = response.reasoning() {
                println!("reasoning: {reasoning_content}");
                messages.push(Message::assistant(reasoning_content).into());
            }

            if let Some(content) = response.message() {
                println!("{content}");
                messages.push(Message::assistant(content).into());
            }
        }
        Ok(())
    }
}

/// Available model providers
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum Model {
    /// DeepSeek model
    #[default]
    Deepseek,
}
