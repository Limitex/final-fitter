use std::process::Command;

use crate::cli::config::DAEMON_BINARY;
use crate::cli::Args;
use crate::error::{CtlError, Result};
use crate::infra::process::{is_running, write_pid};

pub fn execute(args: &Args) -> Result<()> {
    if is_running(&args.pid_file) {
        return Err(CtlError::DaemonAlreadyRunning);
    }

    let child = Command::new(DAEMON_BINARY)
        .spawn()
        .map_err(|e| CtlError::DaemonStartFailed(format!("{} (is {} in PATH?)", e, DAEMON_BINARY)))?;

    write_pid(&args.pid_file, child.id())?;

    println!("Started daemon (PID: {})", child.id());
    Ok(())
}
