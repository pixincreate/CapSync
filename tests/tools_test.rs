use capsync::tools::{all_tools, get_tool};

#[cfg(unix)]
#[test]
fn test_additional_tools_detect_and_sync_skills() {
    use capsync::config::Config;
    use std::{collections::HashMap, fs, process::Command};
    use tempfile::TempDir;

    for (name, config_subpath, skills_subpath) in [
        ("fx", ".fx", ".agents/skills"),
        ("grok", ".grok", ".grok/skills"),
        ("kimchi", ".config/kimchi", ".config/kimchi/harness/skills"),
        ("minimax-code", ".minimax", ".minimax/skills"),
        (
            "posit-assistant",
            ".posit/assistant",
            ".posit/assistant/skills",
        ),
        ("sarvam-code", ".sarvam", ".agents/skills"),
        ("zcode", ".zcode", ".zcode/skills"),
    ] {
        let home = TempDir::new().unwrap();
        let source = home.path().join("source");
        let destination = home.path().join(skills_subpath);
        let config_path = home.path().join(".config/capsync/config.toml");
        fs::create_dir_all(source.join("example")).unwrap();
        fs::write(source.join("example/SKILL.md"), "Example skill").unwrap();
        fs::create_dir_all(config_path.parent().unwrap()).unwrap();
        fs::write(
            &config_path,
            toml::to_string_pretty(&Config {
                skills_source: source.clone(),
                commands_source: None,
                destinations: HashMap::new(),
            })
            .unwrap(),
        )
        .unwrap();

        let run = |args: &[&str]| {
            let output = Command::new(env!("CARGO_BIN_EXE_capsync"))
                .args(args)
                .env("HOME", home.path())
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{name} {args:?}: {}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            String::from_utf8(output.stdout).unwrap()
        };

        fs::create_dir_all(home.path().join(".agents/skills")).unwrap();
        let detected = run(&["detect-tools"]);
        assert!(!detected.lines().any(|line| line == name));
        assert!(!detected.lines().any(|line| line == "fx"));
        assert!(!detected.lines().any(|line| line == "sarvam-code"));

        fs::create_dir_all(home.path().join(config_subpath)).unwrap();
        assert!(
            run(&["detect-tools"]).lines().any(|line| line == name),
            "{name}: tool config directory is not detected"
        );

        run(&["add", name, "--no-sync"]);
        let saved: Config = toml::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap();
        let tool = &saved.destinations[name];
        assert!(tool.enabled);
        assert_eq!(tool.skills_path, destination);
        assert_eq!(tool.commands_path, None);
        assert!(!destination.is_symlink());
        assert!(!destination.join("example/SKILL.md").exists());

        run(&["sync"]);
        assert!(destination.is_symlink(), "{name}: no skills symlink");
        assert_eq!(fs::read_link(&destination).unwrap(), source);
        assert_eq!(
            fs::read_to_string(destination.join("example/SKILL.md")).unwrap(),
            "Example skill"
        );
    }
}

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
fn test_external_supported_tools_exist() {
    let tools = all_tools();
    let names: Vec<_> = tools.iter().map(|t| t.name).collect();

    for name in [
        "aider-desk",
        "antigravity-cli",
        "antigravity-ide",
        "astrbot",
        "autohand-code",
        "bob",
        "claude-code",
        "codearts-agent",
        "codemaker",
        "codestudio",
        "deepagents",
        "devin",
        "dexto",
        "firebender",
        "forgecode",
        "hermes-agent",
        "inference-sh",
        "jazz",
        "kimi-code-cli",
        "lingma",
        "loaf",
        "moxby",
        "ona",
        "qoder-cn",
        "reasonix",
        "rovodev",
        "tabnine-cli",
        "terramind",
        "tinycloud",
        "warp",
        "zed",
        "zenflow",
    ] {
        assert!(names.contains(&name), "missing tool: {name}");
    }
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

    let claude_code = get_tool("claude-code").unwrap();
    assert!(claude_code.config_path.ends_with(".claude"));
    assert!(claude_code.skills_path.ends_with(".claude/skills"));
    assert!(claude_code.commands_path.is_some());

    let devin = get_tool("devin").unwrap();
    assert!(devin.config_path.ends_with(".config/devin"));
    assert!(devin.skills_path.ends_with(".config/devin/skills"));

    let tabnine_cli = get_tool("tabnine-cli").unwrap();
    assert!(tabnine_cli.config_path.ends_with(".tabnine"));
    assert!(tabnine_cli.skills_path.ends_with(".tabnine/agent/skills"));

    let zed = get_tool("zed").unwrap();
    assert!(zed.config_path.ends_with(".config/zed"));
    assert!(zed.skills_path.ends_with(".agents/skills"));
}
