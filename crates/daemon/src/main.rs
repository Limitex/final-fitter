use clap::Parser;
use daemon::cli::Args;
use daemon::config::DaemonConfig;
use daemon::error::Result;
use daemon::server::LockGuard;
use daemon::server::process;
use daemon::{Server, ServerConfig};

fn main() -> Result<()> {
    let args = Args::parse();

    let config = DaemonConfig::load()?.with_foreground(args.foreground);

    // Acquire lock before daemonizing to prevent TOCTOU race
    let _lock_guard = LockGuard::try_acquire(&config.lock_file)?;

    if !config.foreground {
        if process::is_daemon_supported() {
            process::daemonize(&config)?;
        } else {
            eprintln!("Daemon mode not supported on this platform, running in foreground");
        }
    }

    tokio::runtime::Runtime::new()?.block_on(run_server(config))
}

async fn run_server(config: DaemonConfig) -> Result<()> {
    let mut server_config = ServerConfig::new().with_tcp(config.tcp_addr.parse()?);

    #[cfg(unix)]
    {
        server_config = server_config.with_uds(&config.socket);
    }

    Server::new(server_config).run().await
}
