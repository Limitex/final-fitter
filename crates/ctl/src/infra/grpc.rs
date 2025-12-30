use std::path::Path;

use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

use crate::cli::config::{CONNECT_TIMEOUT, UDS_DUMMY_URI};
use crate::cli::Args;
use crate::error::{CtlError, Result};

/// Connect to daemon, preferring UDS over TCP unless --tcp flag is set
pub async fn connect(args: &Args) -> Result<Channel> {
    // --tcp flag forces TCP
    if args.tcp {
        return connect_tcp(&args.tcp_addr).await;
    }

    // Try UDS first
    if args.socket.exists() {
        match connect_uds(&args.socket).await {
            Ok(channel) => return Ok(channel),
            Err(e) => {
                eprintln!("UDS connection failed, falling back to TCP: {}", e);
            }
        }
    }

    // Fallback to TCP
    connect_tcp(&args.tcp_addr).await
}

async fn connect_uds(path: &Path) -> Result<Channel> {
    let path = path.to_path_buf();

    let channel = Endpoint::from_static(UDS_DUMMY_URI)
        .connect_timeout(CONNECT_TIMEOUT)
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

async fn connect_tcp(addr: &str) -> Result<Channel> {
    let channel = Channel::from_shared(addr.to_string())
        .map_err(|e| CtlError::ConnectionFailed(format!("invalid address: {}", e)))?
        .connect_timeout(CONNECT_TIMEOUT)
        .connect()
        .await
        .map_err(|e| CtlError::ConnectionFailed(format!("TCP: {}", e)))?;

    Ok(channel)
}
