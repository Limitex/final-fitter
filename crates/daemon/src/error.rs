use thiserror::Error;

pub type Result<T> = std::result::Result<T, DaemonError>;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error("configuration error: {0}")]
    ConfigError(#[from] Box<figment::Error>),

    #[error("failed to acquire lock: {0}")]
    LockError(String),

    #[error("daemon is already running")]
    AlreadyRunning,

    #[error("failed to daemonize: {0}")]
    DaemonizeError(String),

    #[error("no listeners configured")]
    NoListenersConfigured,

    #[error("failed to build gRPC reflection: {0}")]
    ReflectionError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("invalid address: {0}")]
    InvalidAddress(#[from] std::net::AddrParseError),

    #[error("message cannot be empty")]
    EmptyMessage,
}
