use clap::Parser;

use ctl::cli::{Args, Command};
use ctl::commands;
use ctl::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Command::Start => commands::start(&args).await,
        Command::Stop => commands::stop(&args),
        Command::Status => commands::status(&args),
        Command::Ping { message } => commands::ping(&args, message).await,
    }
}
