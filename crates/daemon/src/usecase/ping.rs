use crate::domain::{self, PingMessage};

pub struct PingUseCase;

impl PingUseCase {
    pub fn new() -> Self {
        Self
    }

    pub fn ping(&self, message: String) -> String {
        let ping = PingMessage::new(message);
        let pong = domain::create_pong(&ping);
        pong.value().to_string()
    }
}

impl Default for PingUseCase {
    fn default() -> Self {
        Self::new()
    }
}
