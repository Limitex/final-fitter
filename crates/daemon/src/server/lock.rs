use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

#[cfg(unix)]
use nix::fcntl::{Flock, FlockArg};

/// A file lock guard that holds an exclusive lock on a lock file.
/// The lock is automatically released when this guard is dropped.
#[cfg(unix)]
pub struct LockGuard {
    _flock: Flock<File>,
}

#[cfg(unix)]
impl LockGuard {
    /// Acquire an exclusive lock on the lock file.
    /// Returns an error if the lock is already held by another process.
    pub fn try_acquire(lock_path: &Path) -> io::Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = lock_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(lock_path)?;

        let flock = Flock::lock(file, FlockArg::LockExclusiveNonblock).map_err(|(_, errno)| {
            if errno == nix::errno::Errno::EWOULDBLOCK {
                io::Error::new(io::ErrorKind::WouldBlock, "daemon is already running")
            } else {
                io::Error::from_raw_os_error(errno as i32)
            }
        })?;

        Ok(Self { _flock: flock })
    }
}

#[cfg(not(unix))]
pub struct LockGuard;

#[cfg(not(unix))]
impl LockGuard {
    pub fn try_acquire(_lock_path: &Path) -> io::Result<Self> {
        Ok(Self)
    }
}
