use std::fs;
use std::path::Path;

use crate::error::{CtlError, Result};

pub fn read_pid(pid_file: &Path) -> Result<i32> {
    let contents = fs::read_to_string(pid_file)
        .map_err(|_| CtlError::PidFileNotFound(pid_file.to_path_buf()))?;

    contents
        .trim()
        .parse()
        .map_err(|_| CtlError::InvalidPid(contents.trim().to_string()))
}

pub fn remove_pid_file(pid_file: &Path) {
    let _ = fs::remove_file(pid_file);
}

pub fn is_running(pid_file: &Path) -> bool {
    read_pid(pid_file).map(process_exists).unwrap_or(false)
}

#[derive(Clone, Copy)]
pub enum Signal {
    Term,
    Kill,
}

#[cfg(unix)]
pub fn send_signal(pid: i32, signal: Signal) -> Result<()> {
    use nix::sys::signal::{Signal as NixSignal, kill};
    use nix::unistd::Pid;

    let sig = match signal {
        Signal::Term => NixSignal::SIGTERM,
        Signal::Kill => NixSignal::SIGKILL,
    };

    kill(Pid::from_raw(pid), sig).map_err(|e| CtlError::SignalFailed(e.to_string()))
}

#[cfg(not(unix))]
pub fn send_signal(_pid: i32, _signal: Signal) -> Result<()> {
    Err(CtlError::UnsupportedPlatform)
}

#[cfg(unix)]
pub fn process_exists(pid: i32) -> bool {
    use nix::sys::signal::kill;
    use nix::unistd::Pid;

    // Signal 0 checks if process exists without sending a signal
    kill(Pid::from_raw(pid), None).is_ok()
}

#[cfg(not(unix))]
pub fn process_exists(_pid: i32) -> bool {
    false
}
