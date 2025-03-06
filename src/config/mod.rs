use serde::Deserialize;
use anyhow::Result;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

pub fn load_config() -> Result<Config> {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let config_str = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    
    Ok(config)
}