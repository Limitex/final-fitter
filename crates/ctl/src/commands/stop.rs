use tracing::{debug, warn};

use crate::config::{CtlConfig, GRACEFUL_SHUTDOWN_ATTEMPTS, SHUTDOWN_POLL_INTERVAL};
use crate::error::{CtlError, Result};
use crate::infra::process::{
    Signal, is_running, process_exists, read_pid, remove_pid_file, send_signal,
};
use crate::{log_success, log_warn};

pub fn execute(config: &CtlConfig) -> Result<()> {
    if !is_running(&config.pid_file) {
        return Err(CtlError::DaemonNotRunning);
    }

    let pid = read_pid(&config.pid_file)?;
    debug!(pid, "Sending SIGTERM for graceful shutdown");
    send_signal(pid, Signal::Term)?;

    for _ in 0..GRACEFUL_SHUTDOWN_ATTEMPTS {
        std::thread::sleep(SHUTDOWN_POLL_INTERVAL);
        if !process_exists(pid) {
            remove_pid_file(&config.pid_file);
            log_success!("Daemon stopped");
            return Ok(());
        }
    }

    warn!(pid, "Graceful shutdown timed out, sending SIGKILL");
    send_signal(pid, Signal::Kill)?;
    remove_pid_file(&config.pid_file);
    log_warn!("Daemon killed");
    Ok(())
}
