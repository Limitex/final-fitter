use std::path::{Path, PathBuf};

use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use serde::{Deserialize, Serialize};

use crate::cli::Args;

/// Default Unix domain socket path
pub const DEFAULT_SOCKET_PATH: &str = "/tmp/ffit-daemon.sock";

/// Default TCP address
pub const DEFAULT_TCP_ADDR: &str = "[::1]:50051";

/// Default PID file path
pub const DEFAULT_PID_FILE: &str = "/tmp/ffit-daemon.pid";

/// Default log file path
pub const DEFAULT_LOG_FILE: &str = "/tmp/ffit-daemon.log";

/// Default working directory for daemon
pub const DEFAULT_WORKDIR: &str = "/";

/// Daemon binary name
pub const DAEMON_BINARY: &str = "ffit-daemon";

/// Environment variable prefix for configuration
pub const ENV_PREFIX: &str = "FFIT_";

/// Application name for config directory
pub const APP_NAME: &str = "ffit";

pub fn default_socket_path() -> PathBuf {
    PathBuf::from(DEFAULT_SOCKET_PATH)
}

pub fn default_pid_file() -> PathBuf {
    PathBuf::from(DEFAULT_PID_FILE)
}

pub fn default_log_file() -> PathBuf {
    PathBuf::from(DEFAULT_LOG_FILE)
}

/// Daemon configuration loaded from multiple sources
///
/// Priority (lowest to highest):
/// 1. Default values
/// 2. Config file (~/.config/ffit/config.toml or /etc/ffit/config.toml)
/// 3. Environment variables (FFIT_ prefix)
/// 4. CLI arguments
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

    /// Log file path (used in daemon mode)
    #[serde(default = "default_log_file")]
    pub log_file: PathBuf,

    /// Working directory for daemon
    #[serde(default = "default_workdir")]
    pub workdir: PathBuf,
}

fn default_tcp_addr() -> String {
    DEFAULT_TCP_ADDR.to_string()
}

fn default_workdir() -> PathBuf {
    PathBuf::from(DEFAULT_WORKDIR)
}

impl Default for DaemonConfig {
    fn default() -> Self {
        Self {
            foreground: false,
            tcp_addr: DEFAULT_TCP_ADDR.to_string(),
            socket: default_socket_path(),
            pid_file: default_pid_file(),
            log_file: default_log_file(),
            workdir: PathBuf::from(DEFAULT_WORKDIR),
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
        let mut figment = Figment::new().merge(Serialized::defaults(DaemonConfig::default()));

        // Add system config file (/etc/ffit/config.toml)
        let system_config = PathBuf::from("/etc").join(APP_NAME).join("config.toml");
        if system_config.exists() {
            figment = figment.merge(Toml::file(&system_config));
        }

        // Add user config file (~/.config/ffit/config.toml)
        if let Some(config_dir) = dirs::config_dir() {
            let user_config = config_dir.join(APP_NAME).join("config.toml");
            if user_config.exists() {
                figment = figment.merge(Toml::file(&user_config));
            }
        }

        // Add environment variables with FFIT_ prefix
        figment = figment.merge(Env::prefixed(ENV_PREFIX).split("_"));

        figment
    }

    /// Merge CLI arguments into the configuration
    pub fn with_cli_args(self, args: &Args) -> Self {
        Self {
            foreground: args.foreground || self.foreground,
            tcp_addr: if args.tcp_addr != DEFAULT_TCP_ADDR {
                args.tcp_addr.clone()
            } else {
                self.tcp_addr
            },
            socket: if args.socket != default_socket_path() {
                args.socket.clone()
            } else {
                self.socket
            },
            pid_file: if args.pid_file != default_pid_file() {
                args.pid_file.clone()
            } else {
                self.pid_file
            },
            log_file: if args.log_file != default_log_file() {
                args.log_file.clone()
            } else {
                self.log_file
            },
            workdir: if args.workdir != Path::new(DEFAULT_WORKDIR) {
                args.workdir.clone()
            } else {
                self.workdir
            },
        }
    }

    /// Get the path to the user config directory
    pub fn user_config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join(APP_NAME))
    }

    /// Get the path to the user config file
    pub fn user_config_file() -> Option<PathBuf> {
        Self::user_config_dir().map(|p| p.join("config.toml"))
    }

    /// Get the path to the system config file
    pub fn system_config_file() -> PathBuf {
        PathBuf::from("/etc").join(APP_NAME).join("config.toml")
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
        assert_eq!(config.workdir, PathBuf::from(DEFAULT_WORKDIR));
    }

    #[test]
    fn test_load_config() {
        // Should load without errors even if no config files exist
        let config = DaemonConfig::load().expect("Failed to load config");
        assert_eq!(config.tcp_addr, DEFAULT_TCP_ADDR);
    }
}
