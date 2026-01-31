use capsync::config::{Config, get_config_path};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default() {
    let config = Config::default();
    assert!(config.source.ends_with("Dev/scripts/skills/skills"));
    // All tools are disabled by default - user enables what they want
    assert!(!config.destinations.get("opencode").unwrap().enabled);
    assert!(!config.destinations.get("claude").unwrap().enabled);
    assert!(!config.destinations.get("codex").unwrap().enabled);
    // Verify we have many tools defined
    assert!(config.destinations.len() > 10);
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

    let opencode = config.destinations.get("opencode").unwrap();
    let loaded_opencode = loaded_config.destinations.get("opencode").unwrap();
    assert_eq!(opencode.path, loaded_opencode.path);
    assert_eq!(opencode.enabled, loaded_opencode.enabled);

    let claude = config.destinations.get("claude").unwrap();
    let loaded_claude = loaded_config.destinations.get("claude").unwrap();
    assert_eq!(claude.path, loaded_claude.path);
}

#[test]
fn test_get_config_path() {
    let path = get_config_path();
    assert!(path.ends_with(".config/capsync/config.toml"));
}

#[test]
fn test_config_validation() {
    let config = Config::default();
    assert!(!config.source.as_os_str().is_empty());
    assert!(
        !config
            .destinations
            .get("opencode")
            .unwrap()
            .path
            .as_os_str()
            .is_empty()
    );
    assert!(
        !config
            .destinations
            .get("claude")
            .unwrap()
            .path
            .as_os_str()
            .is_empty()
    );
}
