use crate::domain::{self, DomainError, PingMessage};

pub struct PingUseCase;

impl PingUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn ping(&self, message: String) -> Result<String, DomainError> {
        let ping = PingMessage::new(message)?;
        let pong = domain::create_pong(&ping);
        Ok(pong.value().to_string())
    }
}

impl Default for PingUseCase {
    fn default() -> Self {
        Self::new()
    }
}
