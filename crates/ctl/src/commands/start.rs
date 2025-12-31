use tokio::process::Command;
use tracing::debug;

use crate::config::{CtlConfig, DAEMON_BINARY, DAEMON_START_POLL_INTERVAL, DAEMON_START_RETRIES};
use crate::error::{CtlError, Result};
use crate::infra::process::{find_daemon_binary, is_running, process_exists, read_pid};
use crate::log_success;

pub async fn execute(config: &CtlConfig) -> Result<()> {
    debug!(pid_file = %config.pid_file.display(), "Checking if daemon is already running");
    if is_running(&config.pid_file) {
        return Err(CtlError::DaemonAlreadyRunning);
    }

    let daemon_path = find_daemon_binary();
    debug!(binary = ?daemon_path, "Spawning daemon process");
    let status = Command::new(&daemon_path).status().await.map_err(|e| {
        CtlError::DaemonStartFailed(format!("{} (is {} in PATH?)", e, DAEMON_BINARY))
    })?;

    if !status.success() {
        return Err(CtlError::DaemonStartFailed(format!(
            "daemon exited with status: {}",
            status
        )));
    }

    debug!("Waiting for daemon to start");
    for _ in 0..DAEMON_START_RETRIES {
        tokio::time::sleep(DAEMON_START_POLL_INTERVAL).await;
        if let Ok(pid) = read_pid(&config.pid_file)
            && process_exists(pid)
        {
            log_success!("Started daemon (PID: {})", pid);
            return Ok(());
        }
    }

    Err(CtlError::DaemonStartFailed(
        "daemon started but PID file not found".to_string(),
    ))
}
