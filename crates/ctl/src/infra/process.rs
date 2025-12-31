use std::ffi::OsString;
use std::fs;
use std::path::Path;

use tracing::warn;

use crate::config::DAEMON_BINARY;
use crate::error::{CtlError, Result};

pub fn read_pid(pid_file: &Path) -> Result<i32> {
    let contents = fs::read_to_string(pid_file).map_err(|_| CtlError::DaemonNotRunning)?;

    contents
        .trim()
        .parse()
        .map_err(|_| CtlError::InvalidPid(contents.trim().to_string()))
}

pub fn remove_pid_file(pid_file: &Path) {
    if let Err(e) = fs::remove_file(pid_file) {
        warn!(path = %pid_file.display(), error = %e, "Failed to remove PID file");
    }
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

#[cfg(windows)]
pub fn send_signal(pid: i32, signal: Signal) -> Result<()> {
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_TERMINATE, TerminateProcess};

    // On Windows, we can only terminate processes (no graceful SIGTERM equivalent)
    // Both Term and Kill signals result in TerminateProcess
    let exit_code: u32 = match signal {
        Signal::Term => 0,
        Signal::Kill => 1,
    };

    // SAFETY:
    // - `OpenProcess` is called with a valid access right flag (PROCESS_TERMINATE)
    //   and a PID obtained from our own PID file. The handle is checked for null
    //   before use.
    // - `TerminateProcess` is called with the valid handle obtained above.
    // - `CloseHandle` is always called to release the handle, preventing leaks.
    //   This is safe even if `TerminateProcess` fails, as the handle remains valid.
    // - All Windows API functions are called with correct parameter types as
    //   specified by windows-sys bindings.
    unsafe {
        let handle: HANDLE = OpenProcess(PROCESS_TERMINATE, 0, pid as u32);
        if handle.is_null() {
            return Err(CtlError::SignalFailed(format!(
                "failed to open process {}: access denied or process not found",
                pid
            )));
        }

        let result = TerminateProcess(handle, exit_code);
        CloseHandle(handle);

        if result == 0 {
            return Err(CtlError::SignalFailed(format!(
                "failed to terminate process {}",
                pid
            )));
        }
    }

    Ok(())
}

#[cfg(unix)]
pub fn process_exists(pid: i32) -> bool {
    use nix::sys::signal::kill;
    use nix::unistd::Pid;

    // Signal 0 checks if process exists without sending a signal
    kill(Pid::from_raw(pid), None).is_ok()
}

#[cfg(windows)]
pub fn process_exists(pid: i32) -> bool {
    use windows_sys::Win32::Foundation::{CloseHandle, WAIT_TIMEOUT};
    use windows_sys::Win32::System::Threading::{
        OpenProcess, PROCESS_SYNCHRONIZE, WaitForSingleObject,
    };

    // SAFETY:
    // - `OpenProcess` is called with PROCESS_SYNCHRONIZE flag, which is the minimum
    //   required access right for `WaitForSingleObject`. The handle is checked for
    //   null before use.
    // - `WaitForSingleObject` is called with timeout 0 (non-blocking check) and a
    //   valid handle. This only queries the process state without modifying it.
    // - `CloseHandle` is always called to release the handle, preventing leaks.
    // - All Windows API functions are called with correct parameter types as
    //   specified by windows-sys bindings.
    unsafe {
        let handle = OpenProcess(PROCESS_SYNCHRONIZE, 0, pid as u32);
        if handle.is_null() {
            return false;
        }

        // WaitForSingleObject with timeout 0 checks if process is still running
        let result = WaitForSingleObject(handle, 0);
        CloseHandle(handle);

        // WAIT_TIMEOUT means process is still running
        result == WAIT_TIMEOUT
    }
}

pub fn find_daemon_binary() -> OsString {
    if let Ok(current_exe) = std::env::current_exe()
        && let Some(dir) = current_exe.parent()
    {
        let sibling = dir.join(DAEMON_BINARY);
        if sibling.exists() {
            return sibling.into_os_string();
        }
    }
    OsString::from(DAEMON_BINARY)
}
