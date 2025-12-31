use crate::config::DaemonConfig;
use crate::error::{DaemonError, Result};

#[cfg(unix)]
pub fn daemonize(config: &DaemonConfig) -> Result<()> {
    use daemonize::Daemonize;
    use std::fs::OpenOptions;

    if let Some(parent) = config.log_file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if let Some(parent) = config.pid_file.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&config.log_file)?;
    let err_file = log_file.try_clone()?;

    Daemonize::new()
        .pid_file(&config.pid_file)
        .chown_pid_file(true)
        .working_directory(&config.workdir)
        .umask(0o027)
        .stdout(log_file)
        .stderr(err_file)
        .start()
        .map_err(|e| DaemonError::DaemonizeError(e.to_string()))?;

    Ok(())
}

#[cfg(not(unix))]
pub fn daemonize(_config: &DaemonConfig) -> Result<()> {
    Err(DaemonError::DaemonizeError(
        "Daemon mode not supported on this platform".to_string(),
    ))
}

pub const fn is_daemon_supported() -> bool {
    cfg!(unix)
}
