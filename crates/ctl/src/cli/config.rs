use std::time::Duration;

// Re-export shared constants from daemon
pub use daemon::cli::config::{
    default_pid_file, default_socket_path, DAEMON_BINARY, DEFAULT_PID_FILE, DEFAULT_SOCKET_PATH,
};

/// Default TCP address (with http:// scheme for client)
pub const DEFAULT_TCP_ADDR: &str = "http://[::1]:50051";

/// Connection timeout for gRPC clients
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

/// Dummy URI for UDS connections (tonic requires a valid URI)
#[cfg(unix)]
pub const UDS_DUMMY_URI: &str = "http://[::1]:50051";

/// Number of attempts for graceful shutdown
pub const GRACEFUL_SHUTDOWN_ATTEMPTS: u32 = 30;

/// Interval between shutdown polling
pub const SHUTDOWN_POLL_INTERVAL: Duration = Duration::from_millis(100);
