use clap::Parser;
use daemon::cli::Args;
use daemon::config::DaemonConfig;
use daemon::error::Result;
use daemon::server::LockGuard;
use daemon::server::process;
use daemon::{Server, ServerConfig};
use tracing::{debug, info, warn};
use tracing_subscriber::EnvFilter;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env().add_directive("daemon=info".parse().unwrap()),
        )
        .init();

    let args = Args::parse();

    debug!("Loading configuration");
    let config = DaemonConfig::load()?.with_foreground(args.foreground);
    debug!(
        tcp_addr = %config.tcp_addr,
        socket = %config.socket.display(),
        pid_file = %config.pid_file.display(),
        foreground = config.foreground,
        "Configuration loaded"
    );

    // Daemonize first, then acquire lock in the child process
    // (flock is not inherited across fork, so we must acquire it after daemonizing)
    if !config.foreground {
        if process::is_daemon_supported() {
            info!("Daemonizing process");
            process::daemonize(&config)?;
        } else {
            warn!("Daemon mode not supported on this platform, running in foreground");
        }
    }

    debug!(lock_file = %config.lock_file.display(), "Acquiring lock");
    let _lock_guard = LockGuard::try_acquire(&config.lock_file)?;
    debug!("Lock acquired");

    info!("Starting server");
    tokio::runtime::Runtime::new()?.block_on(run_server(config))
}

async fn run_server(config: DaemonConfig) -> Result<()> {
    let mut server_config = ServerConfig::default().with_tcp(config.tcp_addr.parse()?);

    #[cfg(unix)]
    {
        server_config = server_config.with_uds(&config.socket);
    }

    Server::new(server_config).run().await
}
