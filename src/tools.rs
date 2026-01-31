use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Tool {
    pub name: &'static str,
    pub config_path: PathBuf,
    pub skills_path: PathBuf,
}

impl Tool {
    fn new(name: &'static str, config_subpath: &str, skills_subpath: &str) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Self {
            name,
            config_path: home.join(config_subpath),
            skills_path: home.join(skills_subpath),
        }
    }
}

pub fn all_tools() -> Vec<Tool> {
    vec![
        Tool::new("adal", ".adal", ".adal/skills"),
        Tool::new("amp", ".config/agents", ".agents/skills"),
        Tool::new("antigravity", ".gemini/antigravity", ".agent/skills"),
        Tool::new("claude", ".claude", ".claude/skills"),
        Tool::new("cline", ".cline", ".cline/skills"),
        Tool::new("codebuddy", ".codebuddy", ".codebuddy/skills"),
        Tool::new("codex", ".codex", ".codex/skills"),
        Tool::new("command-code", ".commandcode", ".commandcode/skills"),
        Tool::new("continue", ".continue", ".continue/skills"),
        Tool::new("crush", ".config/crush", ".config/crush/skills"),
        Tool::new("cursor", ".cursor", ".cursor/skills"),
        Tool::new("droid", ".factory", ".factory/skills"),
        Tool::new("gemini-cli", ".gemini", ".gemini/skills"),
        Tool::new("github-copilot", ".copilot", ".copilot/skills"),
        Tool::new("goose", ".config/goose", ".config/goose/skills"),
        Tool::new("junie", ".junie", ".junie/skills"),
        Tool::new("kilo", ".kilocode", ".kilocode/skills"),
        Tool::new("kimi-cli", ".kimi", ".config/agents/skills"),
        Tool::new("kiro-cli", ".kiro", ".kiro/skills"),
        Tool::new("kode", ".kode", ".kode/skills"),
        Tool::new("mcpjam", ".mcpjam", ".mcpjam/skills"),
        Tool::new("moltbot", ".moltbot", ".moltbot/skills"),
        Tool::new("mux", ".mux", ".mux/skills"),
        Tool::new("neovate", ".neovate", ".neovate/skills"),
        Tool::new("opencode", ".config/opencode", ".config/opencode/skill"),
        Tool::new("openhands", ".openhands", ".openhands/skills"),
        Tool::new("openclaude", ".openclaude", ".openclaude/skills"),
        Tool::new("pi", ".pi/agent", ".pi/agent/skills"),
        Tool::new("pochi", ".pochi", ".pochi/skills"),
        Tool::new("qoder", ".qoder", ".qoder/skills"),
        Tool::new("qwen-code", ".qwen", ".qwen/skills"),
        Tool::new("replit", ".agent", ".agent/skills"),
        Tool::new("roo", ".roo", ".roo/skills"),
        Tool::new("trae", ".trae", ".trae/skills"),
        Tool::new("trae-cn", ".trae-cn", ".trae-cn/skills"),
        Tool::new("windsurf", ".codeium/windsurf", ".codeium/windsurf/skills"),
        Tool::new("zencoder", ".zencoder", ".zencoder/skills"),
    ]
}

pub fn get_tool(name: &str) -> Option<Tool> {
    all_tools().into_iter().find(|t| t.name == name)
}
