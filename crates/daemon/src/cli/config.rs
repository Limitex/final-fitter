use std::path::PathBuf;

/// Default Unix domain socket path
pub const DEFAULT_SOCKET_PATH: &str = "/tmp/ffit-daemon.sock";

/// Default TCP address
pub const DEFAULT_TCP_ADDR: &str = "[::1]:50051";

/// Default PID file path
pub const DEFAULT_PID_FILE: &str = "/tmp/ffit-daemon.pid";

/// Default log file path
pub const DEFAULT_LOG_FILE: &str = "/tmp/ffit-daemon.log";

/// Default working directory for daemon
pub const DEFAULT_WORKDIR: &str = "/";

/// Daemon binary name
pub const DAEMON_BINARY: &str = "ffit-daemon";

pub fn default_socket_path() -> PathBuf {
    PathBuf::from(DEFAULT_SOCKET_PATH)
}

pub fn default_pid_file() -> PathBuf {
    PathBuf::from(DEFAULT_PID_FILE)
}

pub fn default_log_file() -> PathBuf {
    PathBuf::from(DEFAULT_LOG_FILE)
}
