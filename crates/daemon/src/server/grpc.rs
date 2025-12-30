use std::sync::Arc;

use tokio::sync::broadcast;

use crate::di::Container;
use crate::server::listener::{ListenAddr, ListenerStream};
use crate::server::shutdown::wait_for_signal;
use crate::ui::GrpcRouter;

pub struct ServerConfig {
    pub tcp: Option<ListenAddr>,
    pub uds: Option<ListenAddr>,
}

impl ServerConfig {
    pub fn new() -> Self {
        Self {
            tcp: None,
            uds: None,
        }
    }

    pub fn with_tcp(mut self, addr: std::net::SocketAddr) -> Self {
        self.tcp = Some(ListenAddr::tcp(addr));
        self
    }

    pub fn with_uds(mut self, path: impl AsRef<std::path::Path>) -> Self {
        self.uds = Some(ListenAddr::unix(path));
        self
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new(config: ServerConfig) -> Self {
        Self { config }
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let container = Arc::new(Container::new());
        let (shutdown_tx, _) = broadcast::channel::<()>(1);

        let mut handles = Vec::new();

        // TCP
        if let Some(addr) = &self.config.tcp {
            let stream = addr.bind().await?;
            let container = Arc::clone(&container);
            let mut shutdown_rx = shutdown_tx.subscribe();
            let addr_display = addr.to_string();

            println!("Listening on {}", addr_display);

            let handle = tokio::spawn(async move {
                if let ListenerStream::Tcp(listener) = stream {
                    let router = GrpcRouter::build(&container);
                    let result = router
                        .serve_with_incoming_shutdown(listener, async move {
                            let _ = shutdown_rx.recv().await;
                        })
                        .await;

                    if let Err(e) = result {
                        eprintln!("TCP server error: {}", e);
                    }
                }
            });

            handles.push(handle);
        }

        // UDS
        if let Some(addr) = &self.config.uds {
            let stream = addr.bind().await?;
            let container = Arc::clone(&container);
            let mut shutdown_rx = shutdown_tx.subscribe();
            let addr_display = addr.to_string();

            println!("Listening on {}", addr_display);

            let handle = tokio::spawn(async move {
                if let ListenerStream::Unix(listener) = stream {
                    let router = GrpcRouter::build(&container);
                    let result = router
                        .serve_with_incoming_shutdown(listener, async move {
                            let _ = shutdown_rx.recv().await;
                        })
                        .await;

                    if let Err(e) = result {
                        eprintln!("UDS server error: {}", e);
                    }
                }
            });

            handles.push(handle);
        }

        if handles.is_empty() {
            return Err("No listeners configured".into());
        }

        wait_for_signal().await;
        println!("Shutting down gracefully...");

        let _ = shutdown_tx.send(());

        for handle in handles {
            let _ = handle.await;
        }

        if let Some(addr) = &self.config.uds {
            addr.cleanup();
        }

        println!("Server stopped");
        Ok(())
    }
}
