use tokio::process::Command;

use crate::config::{CtlConfig, DAEMON_BINARY, DAEMON_START_POLL_INTERVAL, DAEMON_START_RETRIES};
use crate::error::{CtlError, Result};
use crate::infra::process::{is_running, process_exists, read_pid};

pub async fn execute(config: &CtlConfig) -> Result<()> {
    if is_running(&config.pid_file) {
        return Err(CtlError::DaemonAlreadyRunning);
    }

    let status = Command::new(DAEMON_BINARY).status().await.map_err(|e| {
        CtlError::DaemonStartFailed(format!("{} (is {} in PATH?)", e, DAEMON_BINARY))
    })?;

    if !status.success() {
        return Err(CtlError::DaemonStartFailed(format!(
            "daemon exited with status: {}",
            status
        )));
    }

    for _ in 0..DAEMON_START_RETRIES {
        tokio::time::sleep(DAEMON_START_POLL_INTERVAL).await;
        if let Ok(pid) = read_pid(&config.pid_file)
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
