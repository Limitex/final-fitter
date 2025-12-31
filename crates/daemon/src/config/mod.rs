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

/// Daemon configuration.
///
/// Priority (lowest to highest):
/// 1. Default values
/// 2. Config file (~/.config/ffit/config.toml or /etc/ffit/config.toml)
/// 3. Environment variables (FFIT_ prefix)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonConfig {
    #[serde(default)]
    pub foreground: bool,

    #[serde(default = "default_tcp_addr")]
    pub tcp_addr: String,

    #[serde(default = "default_socket_path")]
    pub socket: PathBuf,

    #[serde(default = "default_pid_file")]
    pub pid_file: PathBuf,

    #[serde(default = "default_lock_file")]
    pub lock_file: PathBuf,

    #[serde(default = "default_log_file")]
    pub log_file: PathBuf,

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
    pub fn load() -> Result<Self, Box<figment::Error>> {
        Self::figment().extract().map_err(Box::new)
    }

    pub fn figment() -> Figment {
        let paths = AppPaths::new();
        let mut figment = Figment::new().merge(Serialized::defaults(DaemonConfig::default()));

        let system_config = paths.system_config_file();
        if system_config.exists() {
            figment = figment.merge(Toml::file(&system_config));
        }

        if let Some(user_config) = paths.user_config_file()
            && user_config.exists()
        {
            figment = figment.merge(Toml::file(&user_config));
        }

        figment = figment.merge(Env::prefixed(ENV_PREFIX).split("_"));

        figment
    }

    pub fn with_foreground(self, foreground: bool) -> Self {
        Self {
            foreground: foreground || self.foreground,
            ..self
        }
    }

    pub fn user_config_dir() -> Option<PathBuf> {
        AppPaths::new().config_dir()
    }

    pub fn user_config_file() -> Option<PathBuf> {
        AppPaths::new().user_config_file()
    }

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
        let config = DaemonConfig::load().expect("Failed to load config");
        assert_eq!(config.tcp_addr, DEFAULT_TCP_ADDR);
    }
}
