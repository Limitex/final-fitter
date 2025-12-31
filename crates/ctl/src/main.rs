use std::process::ExitCode;

use clap::Parser;

use ctl::cli::{Args, Command};
use ctl::commands;
use ctl::config::CtlConfig;

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(e) = run().await {
        eprintln!("Error: {e}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> ctl::error::Result<()> {
    let args = Args::parse();
    let config = CtlConfig::load()?.with_tcp_flag(args.tcp);

    match &args.command {
        Command::Start => commands::start(&config).await,
        Command::Stop => commands::stop(&config),
        Command::Status => commands::status(&config),
        Command::Ping { message } => commands::ping(&config, message).await,
    }
}
