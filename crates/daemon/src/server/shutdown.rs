use tokio::sync::broadcast;
use tracing::{info, warn};

#[derive(Clone)]
pub struct ShutdownSignal {
    sender: broadcast::Sender<()>,
}

impl ShutdownSignal {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1);
        Self { sender }
    }

    pub fn trigger(&self) {
        let _ = self.sender.send(());
    }

    pub fn subscribe(&self) -> broadcast::Receiver<()> {
        self.sender.subscribe()
    }

    pub async fn wait(&self) {
        let mut receiver = self.subscribe();
        let _ = receiver.recv().await;
    }
}

impl Default for ShutdownSignal {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn wait_for_signal() {
    #[cfg(unix)]
    let sigterm = match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
        Ok(signal) => Some(signal),
        Err(e) => {
            warn!(
                "Failed to install SIGTERM handler: {}. SIGTERM will be ignored.",
                e
            );
            None
        }
    };

    #[cfg(unix)]
    {
        let ctrl_c = tokio::signal::ctrl_c();

        if let Some(mut sigterm) = sigterm {
            tokio::select! {
                result = ctrl_c => {
                    match result {
                        Ok(()) => info!("Received Ctrl+C"),
                        Err(e) => warn!("Error waiting for Ctrl+C: {}", e),
                    }
                }
                _ = sigterm.recv() => info!("Received SIGTERM"),
            }
        } else {
            match ctrl_c.await {
                Ok(()) => info!("Received Ctrl+C"),
                Err(e) => warn!("Error waiting for Ctrl+C: {}", e),
            }
        }
    }

    #[cfg(not(unix))]
    {
        match tokio::signal::ctrl_c().await {
            Ok(()) => info!("Received Ctrl+C"),
            Err(e) => warn!("Error waiting for Ctrl+C: {}", e),
        }
    }
}
