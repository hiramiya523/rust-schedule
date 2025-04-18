use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub enabled: bool,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
}

impl Config {
    pub fn load() -> Self {
        let config_path =
            std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/default.toml".to_string());

        let config_str = fs::read_to_string(config_path).expect("Failed to read config file");

        toml::from_str(&config_str).expect("Failed to parse config file")
    }
}
