use thiserror::Error;

pub type Result<T> = std::result::Result<T, CtlError>;

#[derive(Debug, Error)]
pub enum CtlError {
    #[error("configuration error: {0}")]
    ConfigError(#[from] Box<figment::Error>),

    #[error("daemon is already running")]
    DaemonAlreadyRunning,

    #[error("daemon is not running")]
    DaemonNotRunning,

    #[error("failed to start daemon: {0}")]
    DaemonStartFailed(String),

    #[error("invalid PID in file: {0}")]
    InvalidPid(String),

    #[error("failed to send signal to process: {0}")]
    SignalFailed(String),

    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),

    #[error("signal handling not supported on this platform")]
    UnsupportedPlatform,
}
