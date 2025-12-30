use clap::Parser;
use daemon::cli::Args;
use daemon::server::process;
use daemon::{Server, ServerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if !args.foreground {
        if process::is_daemon_supported() {
            process::daemonize(&args)?;
        } else {
            eprintln!("Daemon mode not supported on this platform, running in foreground");
        }
    }

    tokio::runtime::Runtime::new()?.block_on(run_server(args))
}

async fn run_server(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = ServerConfig::new().with_tcp(args.tcp_addr.parse()?);

    #[cfg(unix)]
    {
        config = config.with_uds(&args.socket);
    }

    Server::new(config).run().await
}
