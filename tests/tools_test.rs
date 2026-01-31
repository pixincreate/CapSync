use capsync::tools::{all_tools, get_tool};

#[test]
fn test_all_tools_returns_tools() {
    let tools = all_tools();
    assert!(!tools.is_empty());
    assert!(tools.len() >= 30); // We have 37 tools currently
}

#[test]
fn test_get_tool_valid() {
    let tool = get_tool("opencode");
    assert!(tool.is_some());
    let tool = tool.unwrap();
    assert_eq!(tool.name, "opencode");
}

#[test]
fn test_get_tool_invalid() {
    let tool = get_tool("nonexistent-tool");
    assert!(tool.is_none());
}

#[test]
fn test_tools_have_names() {
    let tools = all_tools();
    for tool in &tools {
        assert!(!tool.name.is_empty());
    }
}

#[test]
fn test_tools_have_paths() {
    let tools = all_tools();
    for tool in &tools {
        assert!(!tool.config_path.as_os_str().is_empty());
        assert!(!tool.skills_path.as_os_str().is_empty());
    }
}

#[test]
fn test_common_tools_exist() {
    let tools = all_tools();
    let names: Vec<_> = tools.iter().map(|t| t.name).collect();

    assert!(names.contains(&"opencode"));
    assert!(names.contains(&"claude"));
    assert!(names.contains(&"codex"));
    assert!(names.contains(&"cursor"));
    assert!(names.contains(&"amp"));
    assert!(names.contains(&"antigravity"));
}
