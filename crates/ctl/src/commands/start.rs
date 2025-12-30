
use tokio::process::Command;

use crate::cli::Args;
use crate::cli::config::{DAEMON_BINARY, DAEMON_START_POLL_INTERVAL, DAEMON_START_RETRIES};
use crate::error::{CtlError, Result};
use crate::infra::process::{is_running, process_exists, read_pid};

pub async fn execute(args: &Args) -> Result<()> {
    if is_running(&args.pid_file) {
        return Err(CtlError::DaemonAlreadyRunning);
    }

    // Daemon handles its own daemonization via daemonize crate
    let status = Command::new(DAEMON_BINARY)
        .arg("--pid-file")
        .arg(&args.pid_file)
        .arg("--socket")
        .arg(&args.socket)
        .arg("--tcp-addr")
        .arg(args.tcp_addr.trim_start_matches("http://"))
        .status()
        .await
        .map_err(|e| {
            CtlError::DaemonStartFailed(format!("{} (is {} in PATH?)", e, DAEMON_BINARY))
        })?;

    if !status.success() {
        return Err(CtlError::DaemonStartFailed(format!(
            "daemon exited with status: {}",
            status
        )));
    }

    // Wait briefly for daemon to start and write PID file
    for _ in 0..DAEMON_START_RETRIES {
        tokio::time::sleep(DAEMON_START_POLL_INTERVAL).await;
        if let Ok(pid) = read_pid(&args.pid_file)
            && process_exists(pid)
        {
            println!("Started daemon (PID: {})", pid);
            return Ok(());
        }
    }

    Err(CtlError::DaemonStartFailed(
        "daemon started but PID file not found".to_string(),
    ))
}
