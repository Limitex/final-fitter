use std::time::Duration;

// Re-export shared constants from daemon
pub use daemon::cli::config::{
    DAEMON_BINARY, DEFAULT_PID_FILE, DEFAULT_SOCKET_PATH, default_pid_file, default_socket_path,
};

/// Default TCP address (with http:// scheme for client)
pub const DEFAULT_TCP_ADDR: &str = "http://[::1]:50051";

/// Connection timeout for gRPC clients
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

/// Dummy URI for UDS connections (tonic requires a valid URI)
#[cfg(unix)]
pub const UDS_DUMMY_URI: &str = "http://[::1]:50051";

/// Number of attempts to verify daemon startup
pub const DAEMON_START_RETRIES: u32 = 10;

/// Interval between daemon startup polling
pub const DAEMON_START_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Number of attempts for graceful shutdown
pub const GRACEFUL_SHUTDOWN_ATTEMPTS: u32 = 30;

/// Interval between shutdown polling
pub const SHUTDOWN_POLL_INTERVAL: Duration = Duration::from_millis(100);
