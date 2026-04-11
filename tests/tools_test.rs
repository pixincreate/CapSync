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
    for tool in tools {
        assert!(!tool.name.is_empty());
    }
}

#[test]
fn test_tools_have_paths() {
    let tools = all_tools();
    for tool in tools {
        assert!(!tool.config_path.as_os_str().is_empty());
        assert!(!tool.skills_path.as_os_str().is_empty());
    }
}

#[test]
fn test_tools_have_commands_paths() {
    let tools = all_tools();

    // Tools that support commands should have commands_path
    let opencode = tools.iter().find(|t| t.name == "opencode").unwrap();
    assert!(opencode.commands_path.is_some());

    let claude = tools.iter().find(|t| t.name == "claude").unwrap();
    assert!(claude.commands_path.is_some());

    let kilo = tools.iter().find(|t| t.name == "kilo").unwrap();
    assert!(kilo.commands_path.is_some());

    let codex = tools.iter().find(|t| t.name == "codex").unwrap();
    assert!(codex.commands_path.is_some());

    // Tools that don't support commands should have None
    let cursor = tools.iter().find(|t| t.name == "cursor").unwrap();
    assert!(cursor.commands_path.is_none());
}

#[test]
fn test_common_tools_exist() {
    let tools = all_tools();
    let names: Vec<_> = tools.iter().map(|t| t.name).collect();

    assert!(names.contains(&"opencode"));
    assert!(names.contains(&"claude"));
    assert!(names.contains(&"codex"));
    assert!(names.contains(&"kilo"));
    assert!(names.contains(&"cursor"));
    assert!(names.contains(&"amp"));
    assert!(names.contains(&"antigravity"));
    assert!(names.contains(&"github-copilot"));
    assert!(names.contains(&"replit"));
    assert!(names.contains(&"pochi"));
    assert!(names.contains(&"neovate"));
}

#[test]
fn test_supported_list_paths_match_registry() {
    let amp = get_tool("amp").unwrap();
    assert!(amp.config_path.ends_with(".config/agents"));
    assert!(amp.skills_path.ends_with(".config/agents/skills"));

    let kimi_cli = get_tool("kimi-cli").unwrap();
    assert!(kimi_cli.config_path.ends_with(".config/agents"));
    assert!(kimi_cli.skills_path.ends_with(".config/agents/skills"));

    let replit = get_tool("replit").unwrap();
    assert!(replit.config_path.ends_with(".config/agents"));
    assert!(replit.skills_path.ends_with(".config/agents/skills"));

    let antigravity = get_tool("antigravity").unwrap();
    assert!(antigravity.config_path.ends_with(".gemini/antigravity"));
    assert!(
        antigravity
            .skills_path
            .ends_with(".gemini/antigravity/skills")
    );

    let github_copilot = get_tool("github-copilot").unwrap();
    assert!(github_copilot.config_path.ends_with(".copilot"));
    assert!(github_copilot.skills_path.ends_with(".copilot/skills"));

    let openclaw = get_tool("openclaw").unwrap();
    assert!(openclaw.config_path.ends_with(".moltbot"));
    assert!(openclaw.skills_path.ends_with(".moltbot/skills"));

    let pi = get_tool("pi").unwrap();
    assert!(pi.config_path.ends_with(".pi/agent"));
    assert!(pi.skills_path.ends_with(".pi/agent/skills"));

    let trae_cn = get_tool("trae-cn").unwrap();
    assert!(trae_cn.config_path.ends_with(".trae-cn"));
    assert!(trae_cn.skills_path.ends_with(".trae-cn/skills"));

    let windsurf = get_tool("windsurf").unwrap();
    assert!(windsurf.config_path.ends_with(".codeium/windsurf"));
    assert!(windsurf.skills_path.ends_with(".codeium/windsurf/skills"));
}
