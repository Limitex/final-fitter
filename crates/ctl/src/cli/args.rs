use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::config::{DEFAULT_PID_FILE, DEFAULT_SOCKET_PATH, DEFAULT_TCP_ADDR};

#[derive(Parser)]
#[command(name = "ffit")]
#[command(about = "ffit-daemon management CLI")]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    /// PID file path
    #[arg(long, global = true, default_value = DEFAULT_PID_FILE)]
    pub pid_file: PathBuf,

    /// Unix socket path
    #[arg(long, global = true, default_value = DEFAULT_SOCKET_PATH)]
    pub socket: PathBuf,

    /// TCP address (used if UDS unavailable)
    #[arg(long, global = true, default_value = DEFAULT_TCP_ADDR)]
    pub tcp_addr: String,

    /// Force TCP connection
    #[arg(long, global = true, default_value = "false")]
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
