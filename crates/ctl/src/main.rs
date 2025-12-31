use std::process::ExitCode;

use clap::Parser;
use tracing_subscriber::EnvFilter;

use ctl::cli::{Args, Command};
use ctl::commands;
use ctl::config::CtlConfig;
use ctl::log_error;

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("ctl=warn".parse().expect("static directive is valid")),
        )
        .init();

    if let Err(e) = run().await {
        log_error!("{}", e);
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

async fn run() -> ctl::error::Result<()> {
    let args = Args::parse();
    let config = CtlConfig::load()?.with_tcp_flag(args.tcp);

    match &args.command {
        Command::Start => commands::start(&config).await,
        Command::Stop => commands::stop(&config).await,
        Command::Status => commands::status(&config).await,
        Command::Ping { message } => commands::ping(&config, message).await,
    }
}
