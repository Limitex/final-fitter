use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use tokio::net::{TcpListener, UnixListener};
use tokio_stream::wrappers::{TcpListenerStream, UnixListenerStream};

pub enum ListenerStream {
    Tcp(TcpListenerStream),
    Unix(UnixListenerStream),
}

#[derive(Debug, Clone)]
pub enum ListenAddr {
    Tcp(SocketAddr),
    Unix(PathBuf),
}

impl ListenAddr {
    pub fn tcp(addr: SocketAddr) -> Self {
        Self::Tcp(addr)
    }

    pub fn unix(path: impl AsRef<Path>) -> Self {
        Self::Unix(path.as_ref().to_path_buf())
    }

    pub async fn bind(&self) -> std::io::Result<ListenerStream> {
        match self {
            Self::Tcp(addr) => {
                let listener = TcpListener::bind(addr).await?;
                Ok(ListenerStream::Tcp(TcpListenerStream::new(listener)))
            }
            Self::Unix(path) => {
                if path.exists() {
                    std::fs::remove_file(path)?;
                }
                let listener = UnixListener::bind(path)?;
                Ok(ListenerStream::Unix(UnixListenerStream::new(listener)))
            }
        }
    }

    pub fn cleanup(&self) {
        if let Self::Unix(path) = self {
            let _ = std::fs::remove_file(path);
        }
    }
}

impl std::fmt::Display for ListenAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tcp(addr) => write!(f, "tcp://{}", addr),
            Self::Unix(path) => write!(f, "unix://{}", path.display()),
        }
    }
}
