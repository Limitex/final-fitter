use std::path::PathBuf;
use std::time::Duration;

use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use serde::{Deserialize, Serialize};

pub use daemon::config::{
    APP_NAME, AppPaths, DAEMON_BINARY, DEFAULT_TCP_ADDR, ENV_PREFIX, default_pid_file,
    default_socket_path, default_tcp_addr,
};

pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
#[cfg(unix)]
pub const UDS_DUMMY_URI: &str = "http://[::1]:50051";
pub const DAEMON_START_RETRIES: u32 = 10;
pub const DAEMON_START_POLL_INTERVAL: Duration = Duration::from_millis(100);
pub const GRACEFUL_SHUTDOWN_ATTEMPTS: u32 = 30;
pub const SHUTDOWN_POLL_INTERVAL: Duration = Duration::from_millis(100);

pub fn to_http_uri(addr: &str) -> String {
    if addr.starts_with("http://") || addr.starts_with("https://") {
        addr.to_string()
    } else {
        format!("http://{}", addr)
    }
}

/// CLI configuration.
///
/// Priority (lowest to highest):
/// 1. Default values
/// 2. Config file (~/.config/ffit/config.toml or /etc/ffit/config.toml)
/// 3. Environment variables (FFIT_ prefix)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtlConfig {
    #[serde(default = "default_pid_file")]
    pub pid_file: PathBuf,

    #[serde(default = "default_socket_path")]
    pub socket: PathBuf,

    #[serde(default = "default_tcp_addr")]
    pub tcp_addr: String,

    #[serde(default)]
    pub tcp: bool,

    #[serde(default = "default_connect_timeout_secs")]
    pub connect_timeout_secs: u64,
}

fn default_connect_timeout_secs() -> u64 {
    CONNECT_TIMEOUT.as_secs()
}

impl Default for CtlConfig {
    fn default() -> Self {
        Self {
            pid_file: default_pid_file(),
            socket: default_socket_path(),
            tcp_addr: default_tcp_addr(),
            tcp: false,
            connect_timeout_secs: CONNECT_TIMEOUT.as_secs(),
        }
    }
}

impl CtlConfig {
    pub fn load() -> Result<Self, Box<figment::Error>> {
        Self::figment().extract().map_err(Box::new)
    }

    pub fn figment() -> Figment {
        let paths = AppPaths::new();
        let mut figment = Figment::new().merge(Serialized::defaults(CtlConfig::default()));

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

    pub fn with_tcp_flag(self, tcp: bool) -> Self {
        Self {
            tcp: tcp || self.tcp,
            ..self
        }
    }

    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_secs)
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
        let config = CtlConfig::default();
        assert_eq!(config.tcp_addr, DEFAULT_TCP_ADDR);
        assert_eq!(config.socket, default_socket_path());
        assert_eq!(config.pid_file, default_pid_file());
        assert!(!config.tcp);
    }

    #[test]
    fn test_load_config() {
        let config = CtlConfig::load().expect("Failed to load config");
        assert_eq!(config.tcp_addr, DEFAULT_TCP_ADDR);
    }

    #[test]
    fn test_to_http_uri() {
        assert_eq!(to_http_uri("[::1]:50051"), "http://[::1]:50051");
        assert_eq!(
            to_http_uri("http://localhost:8080"),
            "http://localhost:8080"
        );
        assert_eq!(to_http_uri("https://example.com"), "https://example.com");
    }
}
