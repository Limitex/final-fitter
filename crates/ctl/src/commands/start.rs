use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::cli::Args;
use crate::cli::config::DAEMON_BINARY;
use crate::error::{CtlError, Result};
use crate::infra::process::{is_running, process_exists, read_pid};

pub fn execute(args: &Args) -> Result<()> {
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
    for _ in 0..10 {
        thread::sleep(Duration::from_millis(100));
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
