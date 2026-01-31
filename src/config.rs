use crate::tools::{all_tools, default_source_path};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub source: PathBuf,
    pub destinations: HashMap<String, DestinationConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DestinationConfig {
    pub enabled: bool,
    pub path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let mut destinations = HashMap::new();

        // Use tools module for all tool definitions
        for tool in all_tools() {
            destinations.insert(
                tool.name.to_string(),
                DestinationConfig {
                    enabled: false, // Default to disabled, let user enable what they want
                    path: tool.skills_path,
                },
            );
        }

        Self {
            source: default_source_path(),
            destinations,
        }
    }
}

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();
    let content = std::fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path();
    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(config)?;
    std::fs::write(config_path, content)?;
    Ok(())
}

pub fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join(".config/capsync/config.toml")
}
