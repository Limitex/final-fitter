use std::sync::Arc;

use crate::di::Container;
use crate::error::{DaemonError, Result};
use crate::server::listener::{ListenAddr, ListenerStream};
use crate::server::shutdown::{ShutdownSignal, wait_for_signal};
use crate::ui::GrpcRouter;

#[derive(Default)]
pub struct ServerConfig {
    pub tcp: Option<ListenAddr>,
    #[cfg(unix)]
    pub uds: Option<ListenAddr>,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tcp(mut self, addr: std::net::SocketAddr) -> Self {
        self.tcp = Some(ListenAddr::tcp(addr));
        self
    }

    #[cfg(unix)]
    pub fn with_uds(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.uds = Some(ListenAddr::unix(path));
        self
    }
}

pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<()> {
        let container = Arc::new(Container::new());
        let shutdown = ShutdownSignal::new();

        let mut handles = Vec::new();

        if let Some(addr) = &self.config.tcp {
            let stream = addr.bind().await?;
            let container = Arc::clone(&container);
            let shutdown = shutdown.clone();
            let addr_display = addr.to_string();

            println!("Listening on {}", addr_display);

            let handle = tokio::spawn(async move {
                if let ListenerStream::Tcp(listener) = stream {
                    let router = GrpcRouter::build(&container);
                    let result = router
                        .serve_with_incoming_shutdown(listener, shutdown.wait())
                        .await;

                    if let Err(e) = result {
                        eprintln!("TCP server error: {}", e);
                    }
                }
            });

            handles.push(handle);
        }

        #[cfg(unix)]
        if let Some(addr) = &self.config.uds {
            let stream = addr.bind().await?;
            let container = Arc::clone(&container);
            let shutdown = shutdown.clone();
            let addr_display = addr.to_string();

            println!("Listening on {}", addr_display);

            let handle = tokio::spawn(async move {
                if let ListenerStream::Unix(listener) = stream {
                    let router = GrpcRouter::build(&container);
                    let result = router
                        .serve_with_incoming_shutdown(listener, shutdown.wait())
                        .await;

                    if let Err(e) = result {
                        eprintln!("UDS server error: {}", e);
                    }
                }
            });

            handles.push(handle);
        }

        if handles.is_empty() {
            return Err(DaemonError::NoListenersConfigured);
        }

        wait_for_signal().await;
        println!("Shutting down gracefully...");

        shutdown.trigger();

        for handle in handles {
            let _ = handle.await;
        }

        #[cfg(unix)]
        if let Some(addr) = &self.config.uds {
            addr.cleanup();
        }

        println!("Server stopped");
        Ok(())
    }
}
