use crate::domain::PingMessage;
use crate::error::DaemonError;

#[derive(Default)]
pub struct PingUseCase;

impl PingUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn ping(&self, message: String) -> Result<String, DaemonError> {
        let ping = PingMessage::new(message)?;
        let pong = ping.to_pong();
        Ok(pong.value().to_string())
    }
}
