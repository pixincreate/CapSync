use cap_sync::config::{get_config_path, Config};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert!(config
        .source
        .directory
        .ends_with("Dev/scripts/skills/skills"));
    assert!(config.tools.opencode.enabled);
    assert!(config.tools.claude.enabled);
    assert!(!config.tools.codex.enabled);
    assert!(config.sync.auto_detect);
    assert!(config.sync.create_dirs);
}

#[test]
fn test_config_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("config.toml");

    let config = Config::default();
    let content = toml::to_string_pretty(&config).unwrap();
    fs::write(&config_path, content).unwrap();

    let loaded_content = fs::read_to_string(&config_path).unwrap();
    let loaded_config: Config = toml::from_str(&loaded_content).unwrap();

    assert_eq!(
        config.tools.opencode.path,
        loaded_config.tools.opencode.path
    );
    assert_eq!(
        config.tools.opencode.enabled,
        loaded_config.tools.opencode.enabled
    );
    assert_eq!(config.tools.claude.path, loaded_config.tools.claude.path);
}

#[test]
fn test_get_config_path() {
    let path = get_config_path();
    assert!(path.ends_with(".config/capsync/config.toml"));
}

#[test]
fn test_config_validation() {
    let config = Config::default();
    assert!(!config.source.directory.as_os_str().is_empty());
    assert!(!config.tools.opencode.path.as_os_str().is_empty());
    assert!(!config.tools.claude.path.as_os_str().is_empty());
}
