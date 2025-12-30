use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum DomainError {
    #[error("invalid message: {0}")]
    InvalidMessage(String),
    #[error("message cannot be empty")]
    EmptyMessage,
}
