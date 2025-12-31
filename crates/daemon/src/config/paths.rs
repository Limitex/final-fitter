use std::path::PathBuf;

use directories::ProjectDirs;

pub const APP_NAME: &str = "ffit";
pub const DAEMON_BINARY: &str = "ffit-daemon";
pub const ENV_PREFIX: &str = "FFIT_";
pub const DEFAULT_TCP_ADDR: &str = "127.0.0.1:50051";
pub const DEFAULT_WORKDIR: &str = "/";

/// XDG-compliant paths on Linux, appropriate paths on macOS/Windows.
/// Falls back to /tmp when runtime directory is not available.
#[derive(Debug, Clone)]
pub struct AppPaths {
    project_dirs: Option<ProjectDirs>,
}

impl AppPaths {
    pub fn new() -> Self {
        Self {
            project_dirs: ProjectDirs::from("", "", APP_NAME),
        }
    }

    /// Linux: $XDG_RUNTIME_DIR/ffit, macOS: ~/Library/Caches/ffit
    pub fn runtime_dir(&self) -> PathBuf {
        self.project_dirs
            .as_ref()
            .and_then(|dirs| dirs.runtime_dir().map(|p| p.to_path_buf()))
            .or_else(|| {
                self.project_dirs
                    .as_ref()
                    .map(|dirs| dirs.cache_dir().to_path_buf())
            })
            .unwrap_or_else(|| PathBuf::from("/tmp"))
    }

    /// Linux: ~/.local/state/ffit, macOS: ~/Library/Application Support/ffit
    pub fn state_dir(&self) -> PathBuf {
        self.project_dirs
            .as_ref()
            .and_then(|dirs| dirs.state_dir().map(|p| p.to_path_buf()))
            .or_else(|| {
                self.project_dirs
                    .as_ref()
                    .map(|dirs| dirs.data_local_dir().to_path_buf())
            })
            .unwrap_or_else(|| PathBuf::from("/tmp"))
    }

    /// Linux: ~/.config/ffit, macOS: ~/Library/Application Support/ffit
    pub fn config_dir(&self) -> Option<PathBuf> {
        self.project_dirs
            .as_ref()
            .map(|dirs| dirs.config_dir().to_path_buf())
    }

    pub fn socket_path(&self) -> PathBuf {
        self.runtime_dir().join(format!("{}.sock", APP_NAME))
    }

    pub fn pid_file(&self) -> PathBuf {
        self.runtime_dir().join(format!("{}.pid", APP_NAME))
    }

    pub fn lock_file(&self) -> PathBuf {
        self.runtime_dir().join(format!("{}.lock", APP_NAME))
    }

    pub fn log_file(&self) -> PathBuf {
        self.state_dir().join(format!("{}.log", APP_NAME))
    }

    pub fn user_config_file(&self) -> Option<PathBuf> {
        self.config_dir().map(|dir| dir.join("config.toml"))
    }

    pub fn system_config_file(&self) -> PathBuf {
        PathBuf::from("/etc").join(APP_NAME).join("config.toml")
    }
}

impl Default for AppPaths {
    fn default() -> Self {
        Self::new()
    }
}

pub fn default_socket_path() -> PathBuf {
    AppPaths::new().socket_path()
}

pub fn default_pid_file() -> PathBuf {
    AppPaths::new().pid_file()
}

pub fn default_lock_file() -> PathBuf {
    AppPaths::new().lock_file()
}

pub fn default_log_file() -> PathBuf {
    AppPaths::new().log_file()
}

pub fn default_workdir() -> PathBuf {
    PathBuf::from(DEFAULT_WORKDIR)
}

pub fn default_tcp_addr() -> String {
    DEFAULT_TCP_ADDR.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_paths_creation() {
        let paths = AppPaths::new();
        let _ = paths.runtime_dir();
        let _ = paths.state_dir();
        let _ = paths.config_dir();
    }

    #[test]
    fn test_default_paths() {
        let paths = AppPaths::new();

        let runtime = paths.runtime_dir();
        assert!(paths.socket_path().starts_with(&runtime));
        assert!(paths.pid_file().starts_with(&runtime));
        assert!(paths.lock_file().starts_with(&runtime));

        let state = paths.state_dir();
        assert!(paths.log_file().starts_with(&state));
    }

    #[test]
    fn test_file_extensions() {
        let paths = AppPaths::new();

        assert!(paths.socket_path().to_string_lossy().ends_with(".sock"));
        assert!(paths.pid_file().to_string_lossy().ends_with(".pid"));
        assert!(paths.lock_file().to_string_lossy().ends_with(".lock"));
        assert!(paths.log_file().to_string_lossy().ends_with(".log"));
    }

    #[test]
    fn test_config_file_name() {
        let paths = AppPaths::new();
        if let Some(config_file) = paths.user_config_file() {
            assert!(config_file.to_string_lossy().ends_with("config.toml"));
        }
    }

    #[test]
    fn test_system_config_file() {
        let paths = AppPaths::new();
        let system_config = paths.system_config_file();
        assert_eq!(system_config, PathBuf::from("/etc/ffit/config.toml"));
    }
}
