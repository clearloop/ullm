use anyhow::Result;
use clap::Parser;
use ullm::cmd::{App, Command};

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::parse();
    app.init_tracing();

    match app.command {
        Command::Chat(chat) => chat.run(app.stream).await?,
    }

    Ok(())
}
