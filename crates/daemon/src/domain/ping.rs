#[derive(Debug, Clone)]
pub struct PingMessage {
    value: String,
}

impl PingMessage {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

pub fn create_pong(message: &PingMessage) -> PingMessage {
    PingMessage::new(format!("pong: {}", message.value()))
}
