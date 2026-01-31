//! Tool definitions - single source of truth for all supported AI tools

use std::path::PathBuf;

/// Represents a tool/agent with its configuration
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: &'static str,
    /// Path where the tool stores its config (used for detection)
    pub config_path: PathBuf,
    /// Path where the tool stores skills (destination for symlinks)
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

/// All supported tools
pub fn all_tools() -> Vec<Tool> {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
    let _config_home = home.join(".config");

    vec![
        // A
        Tool::new("adal", ".adal", ".adal/skills"),
        Tool::new("amp", ".config/agents", ".agents/skills"),
        Tool::new("antigravity", ".gemini/antigravity", ".agent/skills"),
        // C
        Tool::new("claude", ".claude", ".claude/skills"),
        Tool::new("cline", ".cline", ".cline/skills"),
        Tool::new("codebuddy", ".codebuddy", ".codebuddy/skills"),
        Tool::new("codex", ".codex", ".codex/skills"),
        Tool::new("command-code", ".commandcode", ".commandcode/skills"),
        Tool::new("continue", ".continue", ".continue/skills"),
        Tool::new("crush", ".config/crush", ".config/crush/skills"),
        Tool::new("cursor", ".cursor", ".cursor/skills"),
        // D
        Tool::new("droid", ".factory", ".factory/skills"),
        // G
        Tool::new("gemini-cli", ".gemini", ".gemini/skills"),
        Tool::new("github-copilot", ".copilot", ".copilot/skills"),
        Tool::new("goose", ".config/goose", ".config/goose/skills"),
        // J
        Tool::new("junie", ".junie", ".junie/skills"),
        // K
        Tool::new("kilo", ".kilocode", ".kilocode/skills"),
        Tool::new("kimi-cli", ".kimi", ".config/agents/skills"),
        Tool::new("kiro-cli", ".kiro", ".kiro/skills"),
        Tool::new("kode", ".kode", ".kode/skills"),
        // M
        Tool::new("mcpjam", ".mcpjam", ".mcpjam/skills"),
        Tool::new("moltbot", ".moltbot", ".moltbot/skills"),
        Tool::new("mux", ".mux", ".mux/skills"),
        // N
        Tool::new("neovate", ".neovate", ".neovate/skills"),
        // O
        Tool::new("opencode", ".config/opencode", ".config/opencode/skill"),
        Tool::new("openhands", ".openhands", ".openhands/skills"),
        Tool::new("openclaude", ".openclaude", ".openclaude/skills"),
        // P
        Tool::new("pi", ".pi/agent", ".pi/agent/skills"),
        Tool::new("pochi", ".pochi", ".pochi/skills"),
        // Q
        Tool::new("qoder", ".qoder", ".qoder/skills"),
        Tool::new("qwen-code", ".qwen", ".qwen/skills"),
        // R
        Tool::new("replit", ".agent", ".agent/skills"),
        Tool::new("roo", ".roo", ".roo/skills"),
        // T
        Tool::new("trae", ".trae", ".trae/skills"),
        Tool::new("trae-cn", ".trae-cn", ".trae-cn/skills"),
        // W
        Tool::new("windsurf", ".codeium/windsurf", ".codeium/windsurf/skills"),
        // Z
        Tool::new("zencoder", ".zencoder", ".zencoder/skills"),
    ]
}
