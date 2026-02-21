use capsync::config::Config;
use capsync::sync::SyncManager;
use std::fs;
use tempfile::TempDir;

fn create_test_config(
    skills_source: Option<&str>,
    commands_source: Option<&str>,
    tools: &[(&str, bool)],
) -> Config {
    let mut config = Config::default();

    if let Some(skills) = skills_source {
        config.skills_source = std::path::PathBuf::from(skills);
    }

    if let Some(commands) = commands_source {
        config.commands_source = Some(std::path::PathBuf::from(commands));
    }

    for (name, enabled) in tools {
        if let Some(dest) = config.destinations.get_mut(*name) {
            dest.enabled = *enabled;
        }
    }

    config
}

#[test]
fn test_sync_commands_returns_empty_when_no_commands_source() {
    let config = create_test_config(Some("/tmp/skills"), None, &[("claude", true)]);

    let result = SyncManager::sync_commands(&config).unwrap();

    assert!(result.successful.is_empty());
    assert!(result.errors.is_empty());
}

#[test]
fn test_sync_commands_fails_when_source_does_not_exist() {
    let config = create_test_config(
        Some("/tmp/skills"),
        Some("/tmp/nonexistent_commands"),
        &[("claude", true)],
    );

    let result = SyncManager::sync_commands(&config);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err
        .to_string()
        .contains("Commands source directory does not exist"));
}

#[test]
fn test_sync_commands_syncs_to_enabled_destinations() {
    let temp_dir = TempDir::new().unwrap();
    let commands_dir = temp_dir.path().join("commands");
    fs::create_dir_all(&commands_dir).unwrap();

    let config = create_test_config(
        Some("/tmp/skills"),
        Some(commands_dir.to_str().unwrap()),
        &[("claude", true)],
    );

    let result = SyncManager::sync_commands(&config).unwrap();

    assert_eq!(result.successful.len(), 1);
    assert!(result.errors.is_empty());
}

#[test]
fn test_sync_skills_fails_when_source_does_not_exist() {
    let config = create_test_config(Some("/tmp/nonexistent_skills"), None, &[("claude", true)]);

    let result = SyncManager::sync_skills(&config);

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err
        .to_string()
        .contains("Skills source directory does not exist"));
}

#[test]
fn test_sync_skills_syncs_to_enabled_destinations() {
    let temp_dir = TempDir::new().unwrap();
    let skills_dir = temp_dir.path().join("skills");
    fs::create_dir_all(&skills_dir).unwrap();

    let config = create_test_config(
        Some(skills_dir.to_str().unwrap()),
        None,
        &[("claude", true), ("opencode", true)],
    );

    let result = SyncManager::sync_skills(&config).unwrap();

    // May get 1 or 2 successes depending on tool availability
    assert!(result.successful.len() >= 1);
}

#[test]
fn test_sync_all_with_only_skills_source() {
    let temp_dir = TempDir::new().unwrap();
    let skills_dir = temp_dir.path().join("skills");
    fs::create_dir_all(&skills_dir).unwrap();

    let config = create_test_config(
        Some(skills_dir.to_str().unwrap()),
        None,
        &[("claude", true)],
    );

    let result = SyncManager::sync_all(&config).unwrap();

    // Should succeed (skills synced, commands skipped since not configured)
    assert!(result.successful.len() >= 1);
}

#[test]
fn test_sync_all_with_both_skills_and_commands() {
    let temp_dir = TempDir::new().unwrap();
    let skills_dir = temp_dir.path().join("skills");
    let commands_dir = temp_dir.path().join("commands");
    fs::create_dir_all(&skills_dir).unwrap();
    fs::create_dir_all(&commands_dir).unwrap();

    let config = create_test_config(
        Some(skills_dir.to_str().unwrap()),
        Some(commands_dir.to_str().unwrap()),
        &[("claude", true)],
    );

    let result = SyncManager::sync_all(&config).unwrap();

    // May get 1 or 2 successes depending on tool availability
    assert!(result.successful.len() >= 1);
}

#[test]
fn test_sync_all_fails_on_skills_error() {
    let config = create_test_config(
        Some("/tmp/nonexistent_skills"),
        Some("/tmp/commands"),
        &[("claude", true)],
    );

    let result = SyncManager::sync_all(&config);

    assert!(result.is_err());
}

#[test]
fn test_sync_commands_with_disabled_tool() {
    let temp_dir = TempDir::new().unwrap();
    let commands_dir = temp_dir.path().join("commands");
    fs::create_dir_all(&commands_dir).unwrap();

    let config = create_test_config(
        Some("/tmp/skills"),
        Some(commands_dir.to_str().unwrap()),
        &[("claude", false)],
    );

    let result = SyncManager::sync_commands(&config).unwrap();

    assert!(result.successful.is_empty());
    assert!(result.errors.is_empty());
}
