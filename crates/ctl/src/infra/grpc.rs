use std::time::Duration;

use tonic::transport::{Channel, Endpoint};

use crate::config::{CtlConfig, to_http_uri};
use crate::error::{CtlError, Result};

/// Prefer UDS over TCP unless --tcp flag is set.
pub async fn connect(config: &CtlConfig) -> Result<Channel> {
    let timeout = config.connect_timeout();

    if config.tcp {
        return connect_tcp(&config.tcp_addr, timeout).await;
    }

    #[cfg(unix)]
    if config.socket.exists() {
        match connect_uds(&config.socket, timeout).await {
            Ok(channel) => return Ok(channel),
            Err(e) => {
                eprintln!("UDS connection failed, falling back to TCP: {}", e);
            }
        }
    }

    connect_tcp(&config.tcp_addr, timeout).await
}

#[cfg(unix)]
async fn connect_uds(path: &std::path::Path, timeout: Duration) -> Result<Channel> {
    use crate::config::UDS_DUMMY_URI;
    use tonic::transport::Uri;
    use tower::service_fn;

    let path = path.to_path_buf();

    // Tonic requires a URI but ignores it for custom connectors
    let channel = Endpoint::from_static(UDS_DUMMY_URI)
        .connect_timeout(timeout)
        .connect_with_connector(service_fn(move |_: Uri| {
            let path = path.clone();
            async move {
                Ok::<_, std::io::Error>(hyper_util::rt::TokioIo::new(
                    tokio::net::UnixStream::connect(path).await?,
                ))
            }
        }))
        .await
        .map_err(|e| CtlError::ConnectionFailed(format!("UDS: {}", e)))?;

    Ok(channel)
}

async fn connect_tcp(addr: &str, timeout: Duration) -> Result<Channel> {
    let uri = to_http_uri(addr);

    let channel = Channel::from_shared(uri)
        .map_err(|e| CtlError::ConnectionFailed(format!("invalid address: {}", e)))?
        .connect_timeout(timeout)
        .connect()
        .await
        .map_err(|e| CtlError::ConnectionFailed(format!("TCP: {}", e)))?;

    Ok(channel)
}
