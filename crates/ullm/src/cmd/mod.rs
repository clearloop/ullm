//! CLI commands for ullm

mod chat;
mod config;

use clap::{Parser, Subcommand};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
pub use {chat::Chat, config::Config};

/// Unified LLM Interface CLI
#[derive(Debug, Parser)]
#[command(name = "ullm", version, about)]
pub struct App {
    /// Enable streaming mode
    #[arg(short, long, global = true)]
    pub stream: bool,

    /// Verbosity level (use -v, -vv, -vvv, etc.)
    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Subcommand to run
    #[command(subcommand)]
    pub command: Command,
}

/// Available commands
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Chat with an LLM
    Chat(chat::Chat),

    /// Generate a configuration file
    Generate,
}

impl App {
    /// Initialize tracing subscriber based on verbosity
    pub fn init_tracing(&self) {
        let level = match self.verbose {
            0 => Level::INFO,
            1 => Level::DEBUG,
            2 => Level::TRACE,
            _ => Level::TRACE,
        };

        FmtSubscriber::builder().with_max_level(level).init();
    }
}
