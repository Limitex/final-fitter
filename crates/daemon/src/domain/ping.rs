use crate::error::DaemonError;

#[derive(Debug, Clone)]
pub struct PingMessage {
    value: String,
}

impl PingMessage {
    pub fn new(value: impl Into<String>) -> Result<Self, DaemonError> {
        let value = value.into();
        if value.is_empty() {
            return Err(DaemonError::EmptyMessage);
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn to_pong(&self) -> Self {
        Self {
            value: format!("pong: {}", self.value),
        }
    }
}
