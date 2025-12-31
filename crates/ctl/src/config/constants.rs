use std::time::Duration;

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
#[cfg(unix)]
pub const UDS_DUMMY_URI: &str = "http://[::1]:50051";
pub const DAEMON_START_RETRIES: u32 = 10;
pub const DAEMON_START_POLL_INTERVAL: Duration = Duration::from_millis(100);
pub const GRACEFUL_SHUTDOWN_ATTEMPTS: u32 = 30;
pub const SHUTDOWN_POLL_INTERVAL: Duration = Duration::from_millis(100);
