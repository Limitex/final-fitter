use tonic::transport::{Channel, Endpoint};

use crate::cli::Args;
use crate::cli::config::{CONNECT_TIMEOUT, to_http_uri};
use crate::error::{CtlError, Result};

/// Connect to daemon, preferring UDS over TCP unless --tcp flag is set
pub async fn connect(args: &Args) -> Result<Channel> {
    // --tcp flag forces TCP
    if args.tcp {
        return connect_tcp(&args.tcp_addr).await;
    }

    // Try UDS first (Unix only)
    #[cfg(unix)]
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

#[cfg(unix)]
async fn connect_uds(path: &std::path::Path) -> Result<Channel> {
    use crate::cli::config::UDS_DUMMY_URI;
    use tonic::transport::Uri;
    use tower::service_fn;

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
    // Ensure the address has a scheme (http://)
    let uri = to_http_uri(addr);

    let channel = Channel::from_shared(uri)
        .map_err(|e| CtlError::ConnectionFailed(format!("invalid address: {}", e)))?
        .connect_timeout(CONNECT_TIMEOUT)
        .connect()
        .await
        .map_err(|e| CtlError::ConnectionFailed(format!("TCP: {}", e)))?;

    Ok(channel)
}
