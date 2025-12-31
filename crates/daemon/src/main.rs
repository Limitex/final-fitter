use clap::Parser;
use daemon::cli::Args;
use daemon::config::DaemonConfig;
use daemon::server::LockGuard;
use daemon::server::process;
use daemon::{Server, ServerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Load configuration with priority: defaults < config file < env vars
    let config = DaemonConfig::load()
        .map_err(|e| format!("Failed to load configuration: {}", e))?
        .with_foreground(args.foreground);

    // Acquire exclusive lock before daemonizing to prevent TOCTOU race
    let _lock_guard = LockGuard::try_acquire(&config.lock_file)
        .map_err(|e| format!("Failed to acquire lock: {}", e))?;

    if !config.foreground {
        if process::is_daemon_supported() {
            process::daemonize(&config)?;
        } else {
            eprintln!("Daemon mode not supported on this platform, running in foreground");
        }
    }

    // Lock is held for the lifetime of the daemon process
    tokio::runtime::Runtime::new()?.block_on(run_server(config))
}

async fn run_server(config: DaemonConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut server_config = ServerConfig::new().with_tcp(config.tcp_addr.parse()?);

    #[cfg(unix)]
    {
        server_config = server_config.with_uds(&config.socket);
    }

    Server::new(server_config).run().await
}
