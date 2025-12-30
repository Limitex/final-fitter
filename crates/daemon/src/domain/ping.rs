use super::error::DomainError;

#[derive(Debug, Clone)]
pub struct PingMessage {
    value: String,
}

impl PingMessage {
    pub fn new(value: impl Into<String>) -> Result<Self, DomainError> {
        let value = value.into();
        if value.is_empty() {
            return Err(DomainError::EmptyMessage);
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub fn create_pong(message: &PingMessage) -> PingMessage {
    PingMessage::new(format!("pong: {}", message.value())).unwrap()
}
