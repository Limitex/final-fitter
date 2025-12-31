use std::path::PathBuf;
use std::time::Duration;

use figment::Figment;
use figment::providers::{Env, Format, Serialized, Toml};
use serde::{Deserialize, Serialize};

// Re-export shared constants from daemon
pub use daemon::config::{
    APP_NAME, DAEMON_BINARY, DEFAULT_TCP_ADDR, ENV_PREFIX, default_pid_file, default_socket_path,
};

/// Connection timeout for gRPC clients
pub const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);

/// Dummy URI for UDS connections (tonic requires a valid URI)
#[cfg(unix)]
pub const UDS_DUMMY_URI: &str = "http://[::1]:50051";

/// Number of attempts to verify daemon startup
pub const DAEMON_START_RETRIES: u32 = 10;

/// Interval between daemon startup polling
pub const DAEMON_START_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Number of attempts for graceful shutdown
pub const GRACEFUL_SHUTDOWN_ATTEMPTS: u32 = 30;

/// Interval between shutdown polling
pub const SHUTDOWN_POLL_INTERVAL: Duration = Duration::from_millis(100);

/// Convert a host:port address to an HTTP URI
pub fn to_http_uri(addr: &str) -> String {
    if addr.starts_with("http://") || addr.starts_with("https://") {
        addr.to_string()
    } else {
        format!("http://{}", addr)
    }
}

/// Control CLI configuration loaded from multiple sources
///
/// Priority (lowest to highest):
/// 1. Default values
/// 2. Config file (~/.config/ffit/config.toml or /etc/ffit/config.toml)
/// 3. Environment variables (FFIT_ prefix)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtlConfig {
    /// PID file path
    #[serde(default = "default_pid_file")]
    pub pid_file: PathBuf,

    /// Unix socket path
    #[serde(default = "default_socket_path")]
    pub socket: PathBuf,

    /// TCP address
    #[serde(default = "default_tcp_addr")]
    pub tcp_addr: String,

    /// Force TCP connection
    #[serde(default)]
    pub tcp: bool,

    /// Connection timeout in seconds
    #[serde(default = "default_connect_timeout_secs")]
    pub connect_timeout_secs: u64,
}

fn default_tcp_addr() -> String {
    DEFAULT_TCP_ADDR.to_string()
}

fn default_connect_timeout_secs() -> u64 {
    CONNECT_TIMEOUT.as_secs()
}

impl Default for CtlConfig {
    fn default() -> Self {
        Self {
            pid_file: default_pid_file(),
            socket: default_socket_path(),
            tcp_addr: DEFAULT_TCP_ADDR.to_string(),
            tcp: false,
            connect_timeout_secs: CONNECT_TIMEOUT.as_secs(),
        }
    }
}

impl CtlConfig {
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
        let mut figment = Figment::new().merge(Serialized::defaults(CtlConfig::default()));

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

    /// Apply TCP flag from CLI
    pub fn with_tcp_flag(self, tcp: bool) -> Self {
        Self {
            tcp: tcp || self.tcp,
            ..self
        }
    }

    /// Get connection timeout as Duration
    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_secs)
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
        let config = CtlConfig::default();
        assert_eq!(config.tcp_addr, DEFAULT_TCP_ADDR);
        assert_eq!(config.socket, default_socket_path());
        assert_eq!(config.pid_file, default_pid_file());
        assert!(!config.tcp);
    }

    #[test]
    fn test_load_config() {
        // Should load without errors even if no config files exist
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
