use std::env;
use config::{Config, File};
use serde::Deserialize;
use std::error::Error;
use std::sync::Arc;
use once_cell::sync::Lazy;

#[derive(Debug, Deserialize)]
pub struct DefaultConfig {
    pub server_port: u16,
    pub trading_pair: String,
    pub book_depth: u16,
    pub ws_config_retry_max: u16,
}

enum EnvVar {
    ServerPort,
    TradingPair,
    BookDepth,
    WSConfigRetryMax
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub default: DefaultConfig,
}

// Lazy static configuration loading
pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    log::info!("Configuration loading ...");
    match load_config_from_env_or_file() {
        Ok(config) => Arc::new(config),
        Err(e) => {
            log::error!("Failed to load configuration: {:?}", e);
            panic!("Failed to load configuration: {:?}", e);
        }
    }
});

impl EnvVar {
    // Returns the environment variable name as a &str
    fn as_str(&self) -> &str {
        match self {
            EnvVar::ServerPort => "SERVER_PORT",
            EnvVar::TradingPair => "TRADING_PAIR",
            EnvVar::BookDepth => "BOOK_DEPTH",
            EnvVar::WSConfigRetryMax => "WS_CONFIG_RETRY_MAX"
        }
    }

    // Fetches the value from the environment, attempts to parse it into the desired type, or returns the default value
    fn get_value<T: std::str::FromStr + Clone>(&self, default: &T) -> T
    where
        T::Err: std::fmt::Debug,
    {
        env::var(self.as_str())
            .ok()
            .and_then(|val| val.parse::<T>().ok()) // Try to parse the value to type T
            .unwrap_or_else(|| default.clone()) // Clone the default value if parsing fails
    }
}

// Loads the configuration from a file (config.toml)
pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let mut settings = Config::default();

    // Load configuration from file
    settings.merge(File::with_name("resources/config.toml"))?;

    // Parse the configuration into the AppConfig structure
    let app_config: AppConfig = settings.try_into()?;

    Ok(app_config)
}

// Load the configuration from environment variables, overriding values from the file if present
pub fn load_config_from_env_or_file() -> Result<AppConfig, Box<dyn Error>> {
    // First, load the configuration from the file
    let mut config = load_config()?;

    // Override with environment variables if they exist
    config.default.server_port = EnvVar::ServerPort
        .get_value(&config.default.server_port); // u16 for server_port

    config.default.trading_pair = EnvVar::TradingPair
        .get_value(&config.default.trading_pair); // String for trading_pair

    config.default.book_depth = EnvVar::BookDepth
        .get_value(&config.default.book_depth); // u16 for book_depth

    config.default.ws_config_retry_max = EnvVar::WSConfigRetryMax
        .get_value(&config.default.ws_config_retry_max); //u16 for ws retry max

    log::info!("Config loaded: {:?}",config);

    Ok(config)
}