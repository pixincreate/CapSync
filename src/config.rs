//! Configuration management for CapSync
//!
//! This module provides:
//! - Configuration file loading and saving
//! - Default configuration generation
//! - Configuration path resolution

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for CapSync
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// Source skills directory configuration
    pub source: SourceConfig,
    /// Tool-specific configurations
    pub tools: ToolsConfig,
    /// Global sync settings
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SourceConfig {
    pub directory: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolsConfig {
    pub opencode: ToolConfig,
    pub claude: ToolConfig,
    pub codex: ToolConfig,
    pub cursor: ToolConfig,
    pub amp: ToolConfig,
    pub antigravity: ToolConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ToolConfig {
    pub enabled: bool,
    pub path: PathBuf,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SyncConfig {
    pub auto_detect: bool,
    pub create_dirs: bool,
}

impl Default for Config {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));

        Self {
            source: SourceConfig {
                directory: home.join("Dev/scripts/skills/skills"),
            },
            tools: ToolsConfig {
                opencode: ToolConfig {
                    enabled: true,
                    path: home.join(".config/opencode/skill"),
                },
                claude: ToolConfig {
                    enabled: true,
                    path: home.join(".claude/skills"),
                },
                codex: ToolConfig {
                    enabled: false,
                    path: home.join(".codex/skills"),
                },
                cursor: ToolConfig {
                    enabled: false,
                    path: home.join(".cursor/skills"),
                },
                amp: ToolConfig {
                    enabled: false,
                    path: home.join(".agents/skills"),
                },
                antigravity: ToolConfig {
                    enabled: false,
                    path: home.join(".agent/skills"),
                },
            },
            sync: SyncConfig {
                auto_detect: true,
                create_dirs: true,
            },
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
