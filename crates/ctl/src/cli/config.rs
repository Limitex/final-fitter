use std::path::PathBuf;
use std::time::Duration;

/// Default Unix domain socket path
pub const DEFAULT_SOCKET_PATH: &str = "/tmp/ffit-daemon.sock";

/// Default TCP address
pub const DEFAULT_TCP_ADDR: &str = "http://[::1]:50051";

/// Default PID file path
pub const DEFAULT_PID_FILE: &str = "/tmp/ffit-daemon.pid";

/// Connection timeout for gRPC clients
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

/// Dummy URI for UDS connections (tonic requires a valid URI)
pub const UDS_DUMMY_URI: &str = "http://[::1]:50051";

/// Number of attempts for graceful shutdown
pub const GRACEFUL_SHUTDOWN_ATTEMPTS: u32 = 30;

/// Interval between shutdown polling
pub const SHUTDOWN_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Daemon binary name
pub const DAEMON_BINARY: &str = "ffit-daemon";

pub fn default_socket_path() -> PathBuf {
    PathBuf::from(DEFAULT_SOCKET_PATH)
}

pub fn default_pid_file() -> PathBuf {
    PathBuf::from(DEFAULT_PID_FILE)
}
