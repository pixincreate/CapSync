use std::path::PathBuf;
use std::sync::LazyLock;

static ALL_TOOLS_VEC: LazyLock<Vec<Tool>> = LazyLock::new(|| {
    vec![
        Tool::new("adal", ".adal", ".adal/skills", None),
        Tool::new("aider-desk", ".aider-desk", ".aider-desk/skills", None),
        Tool::new("amp", ".config/agents", ".config/agents/skills", None),
        Tool::new(
            "antigravity",
            ".gemini/antigravity",
            ".gemini/antigravity/skills",
            None,
        ),
        Tool::new(
            "antigravity-cli",
            ".gemini/antigravity-cli",
            ".gemini/antigravity-cli/skills",
            None,
        ),
        Tool::new(
            "antigravity-ide",
            ".gemini/antigravity-ide",
            ".gemini/antigravity-ide/skills",
            None,
        ),
        Tool::new("astrbot", ".astrbot", ".astrbot/data/skills", None),
        Tool::new("autohand-code", ".autohand", ".autohand/skills", None),
        Tool::new("augment", ".augment", ".augment/skills", None),
        Tool::new("bob", ".bob", ".bob/skills", None),
        Tool::new(
            "claude",
            ".claude",
            ".claude/skills",
            Some(".claude/commands"),
        ),
        Tool::new(
            "claude-code",
            ".claude",
            ".claude/skills",
            Some(".claude/commands"),
        ),
        Tool::new("cline", ".cline", ".cline/skills", None),
        Tool::new(
            "codearts-agent",
            ".codeartsdoer",
            ".codeartsdoer/skills",
            None,
        ),
        Tool::new("codebuddy", ".codebuddy", ".codebuddy/skills", None),
        Tool::new("codemaker", ".codemaker", ".codemaker/skills", None),
        Tool::new("codestudio", ".codestudio", ".codestudio/skills", None),
        Tool::new("codex", ".codex", ".codex/skills", Some(".codex/commands")),
        Tool::new("command-code", ".commandcode", ".commandcode/skills", None),
        Tool::new("continue", ".continue", ".continue/skills", None),
        Tool::new("cortex", ".snowflake/cortex", ".cortex/skills", None),
        Tool::new("crush", ".config/crush", ".config/crush/skills", None),
        Tool::new("cursor", ".cursor", ".cursor/skills", None),
        Tool::new(
            "deepagents",
            ".deepagents",
            ".deepagents/agent/skills",
            None,
        ),
        Tool::new("devin", ".config/devin", ".config/devin/skills", None),
        Tool::new("dexto", ".dexto", ".agents/skills", None),
        Tool::new("droid", ".factory", ".factory/skills", None),
        Tool::new("firebender", ".firebender", ".firebender/skills", None),
        Tool::new("forgecode", ".forge", ".forge/skills", None),
        Tool::new("gemini-cli", ".gemini", ".gemini/skills", None),
        Tool::new("github-copilot", ".copilot", ".copilot/skills", None),
        Tool::new("goose", ".config/goose", ".config/goose/skills", None),
        Tool::new("hermes-agent", ".hermes", ".hermes/skills", None),
        Tool::new("iflow-cli", ".iflow", ".iflow/skills", None),
        Tool::new("inference-sh", ".inferencesh", ".inferencesh/skills", None),
        Tool::new("jazz", ".jazz", ".jazz/skills", None),
        Tool::new("junie", ".junie", ".junie/skills", None),
        Tool::new(
            "kilo",
            ".kilocode",
            ".kilocode/skills",
            Some(".kilocode/commands"),
        ),
        Tool::new("kimi-cli", ".config/agents", ".config/agents/skills", None),
        Tool::new("kimi-code-cli", ".kimi-code", ".agents/skills", None),
        Tool::new("kiro-cli", ".kiro", ".kiro/skills", None),
        Tool::new("kode", ".kode", ".kode/skills", None),
        Tool::new("lingma", ".lingma", ".lingma/skills", None),
        Tool::new("loaf", ".loaf", ".agents/skills", None),
        Tool::new("mcpjam", ".mcpjam", ".mcpjam/skills", None),
        Tool::new("mistral-vibe", ".vibe", ".vibe/skills", None),
        Tool::new("moltbot", ".moltbot", ".moltbot/skills", None),
        Tool::new("moxby", ".moxby", ".moxby/skills", None),
        Tool::new("mux", ".mux", ".mux/skills", None),
        Tool::new("neovate", ".neovate", ".neovate/skills", None),
        Tool::new("ona", ".ona", ".ona/skills", None),
        Tool::new(
            "opencode",
            ".config/opencode",
            ".config/opencode/skill",
            Some(".config/opencode/commands"),
        ),
        Tool::new("openhands", ".openhands", ".openhands/skills", None),
        Tool::new("openclaw", ".moltbot", ".moltbot/skills", None),
        Tool::new("pi", ".pi/agent", ".pi/agent/skills", None),
        Tool::new("pochi", ".pochi", ".pochi/skills", None),
        Tool::new("qoder", ".qoder", ".qoder/skills", None),
        Tool::new("qoder-cn", ".qoder-cn", ".qoder-cn/skills", None),
        Tool::new("qwen-code", ".qwen", ".qwen/skills", None),
        Tool::new("reasonix", ".reasonix", ".reasonix/skills", None),
        Tool::new("replit", ".config/agents", ".config/agents/skills", None),
        Tool::new("rovodev", ".rovodev", ".rovodev/skills", None),
        Tool::new("roo", ".roo", ".roo/skills", None),
        Tool::new("tabnine-cli", ".tabnine", ".tabnine/agent/skills", None),
        Tool::new("terramind", ".terramind", ".terramind/skills", None),
        Tool::new("tinycloud", ".tinycloud", ".tinycloud/skills", None),
        Tool::new("trae", ".trae", ".trae/skills", None),
        Tool::new("trae-cn", ".trae-cn", ".trae-cn/skills", None),
        Tool::new("warp", ".warp", ".agents/skills", None),
        Tool::new(
            "windsurf",
            ".codeium/windsurf",
            ".codeium/windsurf/skills",
            None,
        ),
        Tool::new("zed", ".config/zed", ".agents/skills", None),
        Tool::new("zencoder", ".zencoder", ".zencoder/skills", None),
        Tool::new("zenflow", ".zencoder", ".zencoder/skills", None),
    ]
});

/// HashMap for O(1) tool lookups by name.
/// NOTE: Depends on ALL_TOOLS_VEC being initialized first (Rust's LazyLock handles this).
static TOOLS_BY_NAME: LazyLock<std::collections::HashMap<&'static str, Tool>> =
    LazyLock::new(|| ALL_TOOLS_VEC.iter().map(|t| (t.name, t.clone())).collect());

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: &'static str,
    pub config_path: PathBuf,
    pub skills_path: PathBuf,
    pub commands_path: Option<PathBuf>,
}

impl Tool {
    fn new(
        name: &'static str,
        config_subpath: &str,
        skills_subpath: &str,
        commands_subpath: Option<&str>,
    ) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Self {
            name,
            config_path: home.join(config_subpath),
            skills_path: home.join(skills_subpath),
            commands_path: commands_subpath.map(|path| home.join(path)),
        }
    }
}

pub fn all_tools() -> &'static Vec<Tool> {
    &ALL_TOOLS_VEC
}

pub fn get_tool(name: &str) -> Option<Tool> {
    TOOLS_BY_NAME.get(name).cloned()
}
