use std::str::FromStr;

use serde::{Deserialize, Serialize};
use super::consts::DEFAULT_LISTEN_ADDRESS;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub listen_address: std::net::Ipv4Addr,
    pub port: u16,
    pub max_connections: u32,
    pub offline_mode: bool,
}

impl Config {
    pub fn default() -> Self {
        Self {
            listen_address: std::net::Ipv4Addr::from_str(DEFAULT_LISTEN_ADDRESS).unwrap(),
            port: 25565,
            max_connections: 20,
            offline_mode: false,
        }
    }

    pub fn from_file(config_file: &std::path::Path) -> anyhow::Result<Self> {
        let config_file_content = std::fs::read_to_string(config_file)?;
        toml::from_str(&config_file_content)
            .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))
    }
}

pub fn create_config_file(config_file: &std::path::Path) -> anyhow::Result<()> {
    let default_config = Config::default();
    let config_content = toml::to_string(&default_config)?;
    match config_file.parent() {
        Some(parent) => {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }
        None => {
            return Err(anyhow::anyhow!("Invalid config file path"));
        }
    };
    std::fs::write(config_file, config_content)?;
    Ok(())
}
