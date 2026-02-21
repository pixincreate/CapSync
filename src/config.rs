use crate::tools::all_tools;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    #[serde(alias = "source", default)] // Added for backwards compatibility. Remove in future.
    pub skills_source: PathBuf,
    #[serde(default)]
    pub commands_source: Option<PathBuf>,
    pub destinations: HashMap<String, DestinationConfig>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DestinationConfig {
    pub enabled: bool,
    #[serde(alias = "path")] // Added for backwards compatibility. Remove in future.
    pub skills_path: PathBuf,
    #[serde(default)]
    pub commands_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        let mut destinations = HashMap::new();

        for tool in all_tools() {
            destinations.insert(
                tool.name.to_string(),
                DestinationConfig {
                    enabled: false, // Default to disabled, let user enable what they want
                    skills_path: tool.skills_path.clone(),
                    commands_path: tool.commands_path.clone(),
                },
            );
        }

        Self {
            skills_source: PathBuf::new(),
            commands_source: None,
            destinations,
        }
    }
}

impl Config {
    pub fn has_commands(&self) -> bool {
        self.commands_source.is_some()
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
