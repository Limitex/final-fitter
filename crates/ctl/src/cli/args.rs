use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ffit")]
#[command(about = "ffit-daemon management CLI")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// Force TCP connection instead of Unix socket
    #[arg(long, global = true)]
    pub tcp: bool,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the daemon
    Start,

    /// Stop the daemon
    Stop,

    /// Check daemon status
    Status,

    /// Ping the daemon
    Ping {
        /// Message to send
        #[arg(default_value = "hello")]
        message: String,
    },
}
