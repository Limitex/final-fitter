use crate::domain::{DomainError, PingMessage};

#[derive(Default)]
pub struct PingUseCase;

impl PingUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn ping(&self, message: String) -> Result<String, DomainError> {
        let ping = PingMessage::new(message)?;
        let pong = ping.to_pong();
        Ok(pong.value().to_string())
    }
}
