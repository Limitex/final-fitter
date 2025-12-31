use std::sync::Arc;

use tonic_reflection::server::v1::ServerReflectionServer;
use tracing::{error, info};

use crate::di::Container;
use crate::error::{DaemonError, Result};
use crate::generated::FILE_DESCRIPTOR_SET;
use crate::generated::ping_service_server::PingServiceServer;
use crate::server::listener::{ListenAddr, ListenerStream};
use crate::server::shutdown::{ShutdownSignal, wait_for_signal};

#[derive(Default)]
pub struct ServerConfig {
    pub tcp: Option<ListenAddr>,
    #[cfg(unix)]
    pub uds: Option<ListenAddr>,
}

impl ServerConfig {
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

        let reflection = build_reflection()?;

        let mut handles = Vec::new();

        if let Some(addr) = &self.config.tcp {
            let stream = addr.bind().await?;
            let container = Arc::clone(&container);
            let reflection = reflection.clone();
            let shutdown = shutdown.clone();
            let addr_display = addr.to_string();

            info!(address = %addr_display, "Listening on TCP");

            let handle = tokio::spawn(async move {
                if let ListenerStream::Tcp(listener) = stream {
                    let router = build_router(&container, reflection);
                    let result = router
                        .serve_with_incoming_shutdown(listener, shutdown.wait())
                        .await;

                    if let Err(e) = result {
                        error!(error = %e, "TCP server error");
                    }
                }
            });

            handles.push(handle);
        }

        #[cfg(unix)]
        if let Some(addr) = &self.config.uds {
            let stream = addr.bind().await?;
            let container = Arc::clone(&container);
            let reflection = reflection.clone();
            let shutdown = shutdown.clone();
            let addr_display = addr.to_string();

            info!(address = %addr_display, "Listening on UDS");

            let handle = tokio::spawn(async move {
                if let ListenerStream::Unix(listener) = stream {
                    let router = build_router(&container, reflection);
                    let result = router
                        .serve_with_incoming_shutdown(listener, shutdown.wait())
                        .await;

                    if let Err(e) = result {
                        error!(error = %e, "UDS server error");
                    }
                }
            });

            handles.push(handle);
        }

        if handles.is_empty() {
            return Err(DaemonError::NoListenersConfigured);
        }

        wait_for_signal().await;
        info!("Shutting down gracefully...");

        shutdown.trigger();

        for handle in handles {
            if let Err(e) = handle.await {
                error!(error = %e, "Listener task panicked during shutdown");
            }
        }

        #[cfg(unix)]
        if let Some(addr) = &self.config.uds {
            addr.cleanup();
        }

        info!("Server stopped");
        Ok(())
    }
}

fn build_reflection()
-> Result<ServerReflectionServer<impl tonic_reflection::server::v1::ServerReflection>> {
    tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1()
        .map_err(|e| DaemonError::ReflectionError(e.to_string()))
}

fn build_router(
    container: &Container,
    reflection: ServerReflectionServer<impl tonic_reflection::server::v1::ServerReflection>,
) -> tonic::transport::server::Router {
    tonic::transport::Server::builder()
        .add_service(reflection)
        .add_service(PingServiceServer::from_arc(container.ping_handler.clone()))
}
