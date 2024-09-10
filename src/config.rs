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
}

enum EnvVar {
    ServerPort,
    TradingPair,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub default: DefaultConfig,
}

pub static CONFIG: Lazy<Arc<AppConfig>> = Lazy::new(|| {
    Arc::new(load_config_from_env_or_file().expect("Failed to load configuration"))
});

impl EnvVar {
    fn as_str(&self) -> &str {
        match self {
            EnvVar::ServerPort => "SERVER_PORT",
            EnvVar::TradingPair => "TRADING_PAIR",
        }
    }

    fn get_value(&self, default: &str) -> String {
        env::var(self.as_str()).unwrap_or_else(|_| default.to_string())
    }
}

pub fn load_config() -> Result<AppConfig, Box<dyn Error>> {
    let mut settings = Config::default();

    // Ładujemy konfigurację z pliku
    settings.merge(File::with_name("resources/config.toml"))?;

    // Parsujemy konfigurację do struktury AppConfig
    let app_config: AppConfig = settings.try_into()?;

    Ok(app_config)
}

pub fn load_config_from_env_or_file() -> Result<AppConfig, Box<dyn Error>> {
    // Najpierw ładujemy konfigurację z pliku
    let mut config = load_config()?;

    // Nadpisujemy zmienne środowiskowe, jeśli istnieją
    config.default.server_port = EnvVar::ServerPort
        .get_value(&config.default.server_port.to_string())
        .parse::<u16>()
        .unwrap_or(config.default.server_port);

    config.default.trading_pair = EnvVar::TradingPair.get_value(&config.default.trading_pair);

    Ok(config)
}