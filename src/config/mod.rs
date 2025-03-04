use serde::Deserialize;
use anyhow::Result;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub stronghold: StrongholdConfig,
    pub iota: IotaConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct StrongholdConfig {
    pub snapshot_path: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct IotaConfig {
    pub node_url: String,
}

pub fn load_config() -> Result<Config> {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let config_str = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    
    Ok(config)
}