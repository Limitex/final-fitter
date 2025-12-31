use std::fs::{File, OpenOptions};
use std::path::Path;

use tracing::{debug, warn};

use crate::error::{DaemonError, Result};

#[cfg(unix)]
use nix::fcntl::{Flock, FlockArg};

/// Exclusive lock on a file, automatically released when dropped.
#[cfg(unix)]
pub struct LockGuard {
    _flock: Flock<File>,
}

#[cfg(unix)]
impl LockGuard {
    pub fn try_acquire(lock_path: &Path) -> Result<Self> {
        if let Some(parent) = lock_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(lock_path)?;

        debug!(path = %lock_path.display(), "Attempting to acquire exclusive lock");
        let flock = Flock::lock(file, FlockArg::LockExclusiveNonblock).map_err(|(_, errno)| {
            if errno == nix::errno::Errno::EWOULDBLOCK {
                warn!(path = %lock_path.display(), "Another daemon instance is already running");
                DaemonError::AlreadyRunning
            } else {
                DaemonError::LockError(errno.to_string())
            }
        })?;

        debug!(path = %lock_path.display(), "Exclusive lock acquired");
        Ok(Self { _flock: flock })
    }
}

#[cfg(not(unix))]
pub struct LockGuard;

#[cfg(not(unix))]
impl LockGuard {
    pub fn try_acquire(_lock_path: &Path) -> Result<Self> {
        Ok(Self)
    }
}
