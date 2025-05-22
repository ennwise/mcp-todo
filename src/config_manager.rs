use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_address: SocketAddr,
    pub quotes_file_path: PathBuf,
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Environment variable parsing error for {variable}: {source}")]
    VarError {
        variable: String,
        source: env::VarError,
    },
    #[error("Invalid server address: {0}")]
    InvalidServerAddress(String),
}

impl From<env::VarError> for ConfigError {
    fn from(err: env::VarError) -> Self {
        // This conversion is tricky because we don't know which variable caused it
        // The caller of `env::var` should ideally map this to a more specific error.
        // For now, we'll use a generic message.
        ConfigError::VarError {
            variable: "Unknown".to_string(),
            source: err,
        }
    }
}

const DEFAULT_SERVER_ADDRESS: &str = "0.0.0.0:8080";
const DEFAULT_QUOTES_FILE_PATH: &str = "data/quotes.json";

pub fn load_config() -> Result<AppConfig, ConfigError> {
    // Attempt to load .env file. It's okay if it's not found (e.g., in production).
    dotenvy::dotenv().ok();

    let server_address_str =
        env::var("RUSTQUOTE_SERVER_ADDRESS").unwrap_or_else(|_| DEFAULT_SERVER_ADDRESS.to_string());

    let server_address = server_address_str.parse::<SocketAddr>().map_err(|e| {
        ConfigError::InvalidServerAddress(format!(
            "Failed to parse server address '{}': {}",
            server_address_str, e
        ))
    })?;

    let quotes_file_path_str = env::var("RUSTQUOTE_QUOTES_FILE_PATH")
        .unwrap_or_else(|_| DEFAULT_QUOTES_FILE_PATH.to_string());

    let quotes_file_path = if PathBuf::from(&quotes_file_path_str).is_absolute() {
        PathBuf::from(quotes_file_path_str)
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(quotes_file_path_str)
    };

    Ok(AppConfig {
        server_address,
        quotes_file_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env; // Added for serial tests

    // Helper to set env var for a test and ensure it's cleaned up
    struct EnvVarGuard {
        key: String,
        original_value: Option<String>,
    }

    impl EnvVarGuard {
        fn new(key: &str, value: &str) -> Self {
            let original_value = env::var(key).ok();
            env::set_var(key, value);
            EnvVarGuard {
                key: key.to_string(),
                original_value,
            }
        }
    }

    impl Drop for EnvVarGuard {
        fn drop(&mut self) {
            if let Some(val) = &self.original_value {
                env::set_var(&self.key, val);
            } else {
                env::remove_var(&self.key);
            }
        }
    }

    #[test]
    #[serial]
    fn test_load_config_defaults() {
        // Ensure no relevant env vars are set that might interfere
        env::remove_var("RUSTQUOTE_SERVER_ADDRESS");
        let _guard_path = EnvVarGuard::new("RUSTQUOTE_QUOTES_FILE_PATH", DEFAULT_QUOTES_FILE_PATH);

        let config = load_config().expect("Failed to load default config");
        assert_eq!(
            config.server_address,
            DEFAULT_SERVER_ADDRESS.parse::<SocketAddr>().unwrap()
        );
        assert_eq!(
            config.quotes_file_path,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_QUOTES_FILE_PATH)
        );
    }

    #[test]
    #[serial]
    fn test_load_config_from_env_vars() {
        let _guard_addr = EnvVarGuard::new("RUSTQUOTE_SERVER_ADDRESS", "127.0.0.1:3000");
        let _guard_path = EnvVarGuard::new("RUSTQUOTE_QUOTES_FILE_PATH", "custom/path.json");

        let config = load_config().expect("Failed to load config from env vars");
        assert_eq!(
            config.server_address,
            "127.0.0.1:3000".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(
            config.quotes_file_path,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("custom/path.json")
        );
    }

    #[test]
    #[serial]
    fn test_load_config_invalid_address_format() {
        let _guard_addr = EnvVarGuard::new("RUSTQUOTE_SERVER_ADDRESS", "invalid-address");
        env::remove_var("RUSTQUOTE_QUOTES_FILE_PATH"); // use default for this

        let result = load_config();
        assert!(matches!(result, Err(ConfigError::InvalidServerAddress(_))));
    }

    #[test]
    #[serial]
    fn test_load_config_partial_env_var_address_set() {
        let _guard_addr = EnvVarGuard::new("RUSTQUOTE_SERVER_ADDRESS", "127.0.0.1:7070");
        let _guard_path = EnvVarGuard::new("RUSTQUOTE_QUOTES_FILE_PATH", DEFAULT_QUOTES_FILE_PATH); // Use default for path

        let config = load_config().expect("Failed to load config with partial env vars");
        assert_eq!(
            config.server_address,
            "127.0.0.1:7070".parse::<SocketAddr>().unwrap()
        );
        assert_eq!(
            config.quotes_file_path,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(DEFAULT_QUOTES_FILE_PATH)
        );
    }

    #[test]
    #[serial]
    fn test_load_config_partial_env_var_path_set() {
        env::remove_var("RUSTQUOTE_SERVER_ADDRESS"); // Use default for address
        let _guard_path =
            EnvVarGuard::new("RUSTQUOTE_QUOTES_FILE_PATH", "specific/quotes_test.json");

        let config = load_config().expect("Failed to load config with partial env vars");
        assert_eq!(
            config.server_address,
            DEFAULT_SERVER_ADDRESS.parse::<SocketAddr>().unwrap()
        );
        assert_eq!(
            config.quotes_file_path,
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("specific/quotes_test.json")
        );
    }
}
