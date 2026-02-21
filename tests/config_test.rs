use capsync::config::{Config, get_config_path};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_default() {
    let config = Config::default();
    // Skills source is empty by default - user must provide their own path
    assert!(config.skills_source.as_os_str().is_empty());
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
    assert_eq!(opencode.skills_path, loaded_opencode.skills_path);
    assert_eq!(opencode.enabled, loaded_opencode.enabled);

    let claude = config.destinations.get("claude").unwrap();
    let loaded_claude = loaded_config.destinations.get("claude").unwrap();
    assert_eq!(claude.skills_path, loaded_claude.skills_path);
}

#[test]
fn test_get_config_path() {
    let path = get_config_path();
    assert!(path.ends_with(".config/capsync/config.toml"));
}

#[test]
fn test_config_validation() {
    let config = Config::default();
    // Skills source is empty by default (user must provide)
    assert!(config.skills_source.as_os_str().is_empty());
    // But destination skills_path should be set
    assert!(
        !config
            .destinations
            .get("opencode")
            .unwrap()
            .skills_path
            .as_os_str()
            .is_empty()
    );
    assert!(
        !config
            .destinations
            .get("claude")
            .unwrap()
            .skills_path
            .as_os_str()
            .is_empty()
    );
}

#[test]
fn test_commands_support_for_tools() {
    let config = Config::default();

    // Claude should have commands path
    let claude = config.destinations.get("claude").unwrap();
    assert!(claude.commands_path.is_some());
    assert!(
        !claude
            .commands_path
            .as_ref()
            .unwrap()
            .as_os_str()
            .is_empty()
    );

    // OpenCode should have commands path
    let opencode = config.destinations.get("opencode").unwrap();
    assert!(opencode.commands_path.is_some());

    // Kilo should have commands path
    let kilo = config.destinations.get("kilo").unwrap();
    assert!(kilo.commands_path.is_some());

    // Codex should have commands path
    let codex = config.destinations.get("codex").unwrap();
    assert!(codex.commands_path.is_some());

    // Cursor should NOT have commands path (not supported)
    let cursor = config.destinations.get("cursor").unwrap();
    assert!(cursor.commands_path.is_none());
}

#[test]
fn test_commands_source_optional() {
    let config = Config::default();
    // Commands source is optional
    assert!(config.commands_source.is_none());
}

#[test]
fn test_backward_compatibility_with_old_config() {
    let old_config = r#"
[destinations.opencode]
enabled = true
path = "/home/user/.config/opencode/skill"

[destinations.claude]
enabled = true
path = "/home/user/.claude/skills"
"#;

    let config: Config = toml::from_str(old_config).unwrap();

    // Old "path" field should map to "skills_path"
    let opencode = config.destinations.get("opencode").unwrap();
    assert_eq!(
        opencode.skills_path.to_string_lossy(),
        "/home/user/.config/opencode/skill"
    );

    // Commands path should be None for old configs
    assert!(opencode.commands_path.is_none());
}
