mod paths;

use std::path::PathBuf;

use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use serde::{Deserialize, Serialize};

pub use paths::{
    APP_NAME, AppPaths, DAEMON_BINARY, DEFAULT_TCP_ADDR, DEFAULT_WORKDIR, ENV_PREFIX,
    default_lock_file, default_log_file, default_pid_file, default_socket_path, default_tcp_addr,
    default_workdir,
};

/// Daemon configuration loaded from multiple sources
///
/// Priority (lowest to highest):
/// 1. Default values
/// 2. Config file (~/.config/ffit/config.toml or /etc/ffit/config.toml)
/// 3. Environment variables (FFIT_ prefix)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    /// Run in foreground (don't daemonize)
    #[serde(default)]
    pub foreground: bool,

    /// TCP listen address
    #[serde(default = "default_tcp_addr")]
    pub tcp_addr: String,

    /// Unix socket path
    #[serde(default = "default_socket_path")]
    pub socket: PathBuf,

    /// PID file path
    #[serde(default = "default_pid_file")]
    pub pid_file: PathBuf,

    /// Lock file path for exclusive daemon instance
    #[serde(default = "default_lock_file")]
    pub lock_file: PathBuf,

    /// Log file path (used in daemon mode)
    #[serde(default = "default_log_file")]
    pub log_file: PathBuf,

    /// Working directory for daemon
    #[serde(default = "default_workdir")]
    pub workdir: PathBuf,
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            foreground: false,
            tcp_addr: default_tcp_addr(),
            socket: default_socket_path(),
            pid_file: default_pid_file(),
            lock_file: default_lock_file(),
            log_file: default_log_file(),
            workdir: default_workdir(),
        }
    }
}

impl DaemonConfig {
    /// Load configuration from all sources with proper priority
    ///
    /// Priority (lowest to highest):
    /// 1. Default values
    /// 2. Config file (~/.config/ffit/config.toml or /etc/ffit/config.toml)
    /// 3. Environment variables (FFIT_ prefix)
    pub fn load() -> Result<Self, Box<figment::Error>> {
        Self::figment().extract().map_err(Box::new)
    }

    /// Create a Figment instance with all configuration sources
    pub fn figment() -> Figment {
        let paths = AppPaths::new();
        let mut figment = Figment::new().merge(Serialized::defaults(DaemonConfig::default()));

        // Add system config file (/etc/ffit/config.toml)
        let system_config = paths.system_config_file();
        if system_config.exists() {
            figment = figment.merge(Toml::file(&system_config));
        }

        // Add user config file (~/.config/ffit/config.toml)
        if let Some(user_config) = paths.user_config_file()
            && user_config.exists()
        {
            figment = figment.merge(Toml::file(&user_config));
        }

        // Add environment variables with FFIT_ prefix
        figment = figment.merge(Env::prefixed(ENV_PREFIX).split("_"));

        figment
    }

    /// Apply foreground flag from CLI
    pub fn with_foreground(self, foreground: bool) -> Self {
        Self {
            foreground: foreground || self.foreground,
            ..self
        }
    }

    /// Get the path to the user config directory
    pub fn user_config_dir() -> Option<PathBuf> {
        AppPaths::new().config_dir()
    }

    /// Get the path to the user config file
    pub fn user_config_file() -> Option<PathBuf> {
        AppPaths::new().user_config_file()
    }

    /// Get the path to the system config file
    pub fn system_config_file() -> PathBuf {
        AppPaths::new().system_config_file()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DaemonConfig::default();
        assert!(!config.foreground);
        assert_eq!(config.tcp_addr, DEFAULT_TCP_ADDR);
        assert_eq!(config.socket, default_socket_path());
        assert_eq!(config.pid_file, default_pid_file());
        assert_eq!(config.log_file, default_log_file());
        assert_eq!(config.workdir, default_workdir());
    }

    #[test]
    fn test_load_config() {
        // Should load without errors even if no config files exist
        let config = DaemonConfig::load().expect("Failed to load config");
        assert_eq!(config.tcp_addr, DEFAULT_TCP_ADDR);
    }
}
