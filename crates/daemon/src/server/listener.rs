use std::net::SocketAddr;
#[cfg(unix)]
use std::path::{Path, PathBuf};

use tokio::net::TcpListener;
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio_stream::wrappers::TcpListenerStream;
#[cfg(unix)]
use tokio_stream::wrappers::UnixListenerStream;
use tracing::debug;

pub enum ListenerStream {
    Tcp(TcpListenerStream),
    #[cfg(unix)]
    Unix(UnixListenerStream),
}

#[derive(Debug, Clone)]
pub enum ListenAddr {
    Tcp(SocketAddr),
    #[cfg(unix)]
    Unix(PathBuf),
}

impl ListenAddr {
    pub fn tcp(addr: SocketAddr) -> Self {
        Self::Tcp(addr)
    }

    #[cfg(unix)]
    pub fn unix(path: impl AsRef<Path>) -> Self {
        Self::Unix(path.as_ref().to_path_buf())
    }

    pub async fn bind(&self) -> std::io::Result<ListenerStream> {
        match self {
            Self::Tcp(addr) => {
                debug!(address = %addr, "Binding TCP listener");
                let listener = TcpListener::bind(addr).await?;
                Ok(ListenerStream::Tcp(TcpListenerStream::new(listener)))
            }
            #[cfg(unix)]
            Self::Unix(path) => {
                if path.exists() {
                    debug!(path = %path.display(), "Removing stale socket");
                    std::fs::remove_file(path)?;
                }
                debug!(path = %path.display(), "Binding Unix socket");
                let listener = UnixListener::bind(path)?;
                Ok(ListenerStream::Unix(UnixListenerStream::new(listener)))
            }
        }
    }

    #[cfg(unix)]
    pub fn cleanup(&self) {
        if let Self::Unix(path) = self {
            debug!(path = %path.display(), "Cleaning up Unix socket");
            let _ = std::fs::remove_file(path);
        }
    }
}

impl std::fmt::Display for ListenAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tcp(addr) => write!(f, "tcp://{}", addr),
            #[cfg(unix)]
            Self::Unix(path) => write!(f, "unix://{}", path.display()),
        }
    }
}
