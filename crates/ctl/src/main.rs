use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, process};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use daemon::generated::{PingRequest, ping_service_client::PingServiceClient};
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

const DEFAULT_UDS_PATH: &str = "/tmp/ffit-daemon.sock";
const DEFAULT_TCP_ADDR: &str = "http://[::1]:50051";

#[derive(Parser)]
#[command(name = "ffit")]
#[command(about = "ffit-daemon management CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,

    /// PID file path
    #[arg(long, global = true, default_value = "/tmp/ffit-daemon.pid")]
    pub pid_file: PathBuf,

    /// Unix socket path
    #[arg(long, global = true, default_value = DEFAULT_UDS_PATH)]
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Command::Start => start_daemon(&cli),
        Command::Stop => stop_daemon(&cli),
        Command::Status => show_status(&cli),
        Command::Ping { message } => {
            require_running(&cli)?;
            ping_server(&cli, message).await
        }
    }
}

/// Ensure daemon is running before making requests
fn require_running(cli: &Cli) -> Result<()> {
    if !is_running(&cli.pid_file) {
        anyhow::bail!("Daemon is not running. Start it with: ffit start");
    }
    Ok(())
}

fn start_daemon(cli: &Cli) -> Result<()> {
    if is_running(&cli.pid_file) {
        anyhow::bail!("Daemon is already running");
    }

    let child = process::Command::new("ffit-daemon")
        .spawn()
        .context("Failed to start daemon. Is ffit-daemon in PATH?")?;

    fs::write(&cli.pid_file, child.id().to_string()).context("Failed to write PID file")?;

    println!("Started daemon (PID: {})", child.id());
    Ok(())
}

fn stop_daemon(cli: &Cli) -> Result<()> {
    let pid = read_pid(&cli.pid_file)?;

    send_signal(pid, Signal::Term)?;

    for _ in 0..30 {
        std::thread::sleep(Duration::from_millis(100));
        if !process_exists(pid) {
            let _ = fs::remove_file(&cli.pid_file);
            println!("Daemon stopped");
            return Ok(());
        }
    }

    send_signal(pid, Signal::Kill)?;
    let _ = fs::remove_file(&cli.pid_file);
    println!("Daemon killed");
    Ok(())
}

fn show_status(cli: &Cli) -> Result<()> {
    match read_pid(&cli.pid_file) {
        Ok(pid) if process_exists(pid) => {
            println!("Running (PID: {})", pid);
        }
        _ => {
            println!("Not running");
        }
    }
    Ok(())
}

async fn ping_server(cli: &Cli, message: &str) -> Result<()> {
    let channel = connect(cli).await?;

    let mut client = PingServiceClient::new(channel);
    let response = client
        .ping(tonic::Request::new(PingRequest {
            message: message.to_string(),
        }))
        .await
        .context("Ping failed")?;

    println!("{}", response.into_inner().message);
    Ok(())
}

async fn connect(cli: &Cli) -> Result<Channel> {
    // --tcp flag forces TCP
    if cli.tcp {
        return connect_tcp(&cli.tcp_addr).await;
    }

    // Try UDS first
    if cli.socket.exists() {
        match connect_uds(&cli.socket).await {
            Ok(channel) => return Ok(channel),
            Err(e) => {
                eprintln!("UDS connection failed, falling back to TCP: {}", e);
            }
        }
    }

    // Fallback to TCP
    connect_tcp(&cli.tcp_addr).await
}

async fn connect_uds(path: &Path) -> Result<Channel> {
    let path = path.to_path_buf();

    // tonic requires a valid URI even for UDS
    let channel = Endpoint::from_static("http://[::]:50051")
        .connect_timeout(Duration::from_secs(3))
        .connect_with_connector(service_fn(move |_: Uri| {
            let path = path.clone();
            async move {
                Ok::<_, std::io::Error>(hyper_util::rt::TokioIo::new(
                    tokio::net::UnixStream::connect(path).await?,
                ))
            }
        }))
        .await
        .context("Failed to connect via UDS")?;

    Ok(channel)
}

async fn connect_tcp(addr: &str) -> Result<Channel> {
    let channel = Channel::from_shared(addr.to_string())?
        .connect_timeout(Duration::from_secs(3))
        .connect()
        .await
        .context("Failed to connect via TCP")?;

    Ok(channel)
}

fn read_pid(pid_file: &Path) -> Result<i32> {
    let contents = fs::read_to_string(pid_file).context("PID file not found")?;
    contents.trim().parse().context("Invalid PID")
}

fn is_running(pid_file: &Path) -> bool {
    read_pid(pid_file).map(process_exists).unwrap_or(false)
}

enum Signal {
    Term,
    Kill,
}

#[cfg(unix)]
fn send_signal(pid: i32, signal: Signal) -> Result<()> {
    use nix::sys::signal::{Signal as NixSignal, kill};
    use nix::unistd::Pid;

    let sig = match signal {
        Signal::Term => NixSignal::SIGTERM,
        Signal::Kill => NixSignal::SIGKILL,
    };
    kill(Pid::from_raw(pid), sig).context("Failed to send signal")
}

#[cfg(not(unix))]
fn send_signal(_pid: i32, _signal: Signal) -> Result<()> {
    anyhow::bail!("Signal handling not supported on this platform")
}

#[cfg(unix)]
fn process_exists(pid: i32) -> bool {
    use nix::sys::signal::kill;
    use nix::unistd::Pid;
    kill(Pid::from_raw(pid), None).is_ok()
}

#[cfg(not(unix))]
fn process_exists(_pid: i32) -> bool {
    false
}
