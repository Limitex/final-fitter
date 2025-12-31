use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CtlError>;

#[derive(Debug, Error)]
pub enum CtlError {
    #[error("configuration error: {0}")]
    ConfigError(#[from] Box<figment::Error>),

    #[error("daemon is not running (start with: ffit start)")]
    DaemonNotRunning,

    #[error("no listeners configured")]
    NoListenersConfigured,

    #[error("failed to start daemon: {0}")]
    DaemonStartFailed(String),

    #[error("PID file not found: {}", .0.display())]
    PidFileNotFound(PathBuf),

    #[error("invalid PID in file: {0}")]
    InvalidPid(String),

    #[error("failed to send signal to process: {0}")]
    SignalFailed(String),

    #[error("connection failed: {0}")]
    ConnectionFailed(String),

    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),

    #[error("transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("signal handling not supported on this platform")]
    UnsupportedPlatform,
}
