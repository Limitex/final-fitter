use clap::Parser;

use ctl::cli::{Args, Command};
use ctl::commands;
use ctl::config::CtlConfig;
use ctl::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Load configuration with priority: defaults < config file < env vars < CLI args
    let config = CtlConfig::load()?.with_cli_args(&args);

    match &args.command {
        Command::Start => commands::start(&config).await,
        Command::Stop => commands::stop(&config),
        Command::Status => commands::status(&config),
        Command::Ping { message } => commands::ping(&config, message).await,
    }
}
