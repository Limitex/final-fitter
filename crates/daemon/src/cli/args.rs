use std::path::PathBuf;

use clap::Parser;

use super::config::{
    DEFAULT_LOG_FILE, DEFAULT_PID_FILE, DEFAULT_SOCKET_PATH, DEFAULT_TCP_ADDR, DEFAULT_WORKDIR,
};

#[derive(Parser, Debug)]
#[command(name = "ffit-daemon")]
#[command(about = "ffit daemon process")]
pub struct Args {
    /// Run in foreground (don't daemonize)
    #[arg(short, long)]
    pub foreground: bool,

    /// TCP listen address
    #[arg(long, default_value = DEFAULT_TCP_ADDR)]
    pub tcp_addr: String,

    /// Unix socket path
    #[arg(long, default_value = DEFAULT_SOCKET_PATH)]
    pub socket: PathBuf,

    /// PID file path
    #[arg(long, default_value = DEFAULT_PID_FILE)]
    pub pid_file: PathBuf,

    /// Log file path (used in daemon mode)
    #[arg(long, default_value = DEFAULT_LOG_FILE)]
    pub log_file: PathBuf,

    /// Working directory for daemon
    #[arg(long, default_value = DEFAULT_WORKDIR)]
    pub workdir: PathBuf,
}
