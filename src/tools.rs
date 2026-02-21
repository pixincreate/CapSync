use std::path::PathBuf;
use std::sync::LazyLock;

static ALL_TOOLS_VEC: LazyLock<Vec<Tool>> = LazyLock::new(|| {
    vec![
        Tool::new("adal", ".adal", ".adal/skills", None),
        Tool::new("amp", ".config/agents", ".agents/skills", None),
        Tool::new("antigravity", ".gemini/antigravity", ".agent/skills", None),
        Tool::new("augment", ".augment", ".augment/skills", None),
        Tool::new(
            "claude",
            ".claude",
            ".claude/skills",
            Some(".claude/commands"),
        ),
        Tool::new("cline", ".cline", ".cline/skills", None),
        Tool::new("codebuddy", ".codebuddy", ".codebuddy/skills", None),
        Tool::new("codex", ".codex", ".codex/skills", Some(".codex/commands")),
        Tool::new("command-code", ".commandcode", ".commandcode/skills", None),
        Tool::new("continue", ".continue", ".continue/skills", None),
        Tool::new("cortex", ".snowflake/cortex", ".cortex/skills", None),
        Tool::new("crush", ".config/crush", ".config/crush/skills", None),
        Tool::new("cursor", ".cursor", ".cursor/skills", None),
        Tool::new("droid", ".factory", ".factory/skills", None),
        Tool::new("gemini-cli", ".gemini", ".gemini/skills", None),
        Tool::new("github-copilot", ".copilot", ".agents/skills", None),
        Tool::new("goose", ".config/goose", ".config/goose/skills", None),
        Tool::new("iflow-cli", ".iflow", ".iflow/skills", None),
        Tool::new("junie", ".junie", ".junie/skills", None),
        Tool::new(
            "kilo",
            ".kilocode",
            ".kilocode/skills",
            Some(".kilocode/commands"),
        ),
        Tool::new("kimi-cli", ".kimi", ".agents/skills", None),
        Tool::new("kiro-cli", ".kiro", ".kiro/skills", None),
        Tool::new("kode", ".kode", ".kode/skills", None),
        Tool::new("mcpjam", ".mcpjam", ".mcpjam/skills", None),
        Tool::new("mistral-vibe", ".vibe", ".vibe/skills", None),
        Tool::new("moltbot", ".moltbot", ".moltbot/skills", None),
        Tool::new("mux", ".mux", ".mux/skills", None),
        Tool::new("neovate", ".neovate", ".neovate/skills", None),
        Tool::new(
            "opencode",
            ".config/opencode",
            ".config/opencode/skill",
            Some(".config/opencode/commands"),
        ),
        Tool::new("openhands", ".openhands", ".openhands/skills", None),
        Tool::new("openclaw", ".openclaw", ".openclaw/skills", None),
        Tool::new("pi", ".pi", ".pi/skills", None),
        Tool::new("pochi", ".pochi", ".pochi/skills", None),
        Tool::new("qoder", ".qoder", ".qoder/skills", None),
        Tool::new("qwen-code", ".qwen", ".qwen/skills", None),
        Tool::new("replit", ".config/agents", ".agents/skills", None),
        Tool::new("roo", ".roo", ".roo/skills", None),
        Tool::new("trae", ".trae", ".trae/skills", None),
        Tool::new("trae-cn", ".trae-cn", ".trae/skills", None),
        Tool::new("windsurf", ".codeium/windsurf", ".windsurf/skills", None),
        Tool::new("zencoder", ".zencoder", ".zencoder/skills", None),
    ]
});

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
