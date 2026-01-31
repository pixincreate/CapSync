//! Tool definitions - single source of truth for all supported AI tools

use std::path::PathBuf;

/// Represents a tool/agent with its configuration
#[derive(Debug, Clone)]
pub struct Tool {
    pub name: &'static str,
    pub display_name: &'static str,
    /// Path where the tool stores its config (used for detection)
    pub config_path: PathBuf,
    /// Path where the tool stores skills (destination for symlinks)
    pub skills_path: PathBuf,
}

impl Tool {
    fn new(
        name: &'static str,
        display_name: &'static str,
        config_subpath: &str,
        skills_subpath: &str,
    ) -> Self {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Self {
            name,
            display_name,
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
        Tool::new("adal", "AdaL", ".adal", ".adal/skills"),
        Tool::new("amp", "Amp", ".config/agents", ".agents/skills"),
        Tool::new(
            "antigravity",
            "Antigravity",
            ".gemini/antigravity",
            ".agent/skills",
        ),
        // C
        Tool::new("claude", "Claude Code", ".claude", ".claude/skills"),
        Tool::new("cline", "Cline", ".cline", ".cline/skills"),
        Tool::new("codebuddy", "CodeBuddy", ".codebuddy", ".codebuddy/skills"),
        Tool::new("codex", "Codex", ".codex", ".codex/skills"),
        Tool::new(
            "command-code",
            "Command Code",
            ".commandcode",
            ".commandcode/skills",
        ),
        Tool::new("continue", "Continue", ".continue", ".continue/skills"),
        Tool::new("crush", "Crush", ".config/crush", ".config/crush/skills"),
        Tool::new("cursor", "Cursor", ".cursor", ".cursor/skills"),
        // D
        Tool::new("droid", "Droid", ".factory", ".factory/skills"),
        // G
        Tool::new("gemini-cli", "Gemini CLI", ".gemini", ".gemini/skills"),
        Tool::new(
            "github-copilot",
            "GitHub Copilot",
            ".copilot",
            ".copilot/skills",
        ),
        Tool::new("goose", "Goose", ".config/goose", ".config/goose/skills"),
        // J
        Tool::new("junie", "Junie", ".junie", ".junie/skills"),
        // K
        Tool::new("kilo", "Kilo Code", ".kilocode", ".kilocode/skills"),
        Tool::new(
            "kimi-cli",
            "Kimi Code CLI",
            ".kimi",
            ".config/agents/skills",
        ),
        Tool::new("kiro-cli", "Kiro CLI", ".kiro", ".kiro/skills"),
        Tool::new("kode", "Kode", ".kode", ".kode/skills"),
        // M
        Tool::new("mcpjam", "MCPJam", ".mcpjam", ".mcpjam/skills"),
        Tool::new("moltbot", "Moltbot", ".moltbot", ".moltbot/skills"),
        Tool::new("mux", "Mux", ".mux", ".mux/skills"),
        // N
        Tool::new("neovate", "Neovate", ".neovate", ".neovate/skills"),
        // O
        Tool::new(
            "opencode",
            "OpenCode",
            ".config/opencode",
            ".config/opencode/skill",
        ),
        Tool::new("openhands", "OpenHands", ".openhands", ".openhands/skills"),
        Tool::new(
            "openclaude",
            "OpenClaude IDE",
            ".openclaude",
            ".openclaude/skills",
        ),
        // P
        Tool::new("pi", "Pi", ".pi/agent", ".pi/agent/skills"),
        Tool::new("pochi", "Pochi", ".pochi", ".pochi/skills"),
        // Q
        Tool::new("qoder", "Qoder", ".qoder", ".qoder/skills"),
        Tool::new("qwen-code", "Qwen Code", ".qwen", ".qwen/skills"),
        // R
        Tool::new("replit", "Replit", ".agent", ".agent/skills"),
        Tool::new("roo", "Roo Code", ".roo", ".roo/skills"),
        // T
        Tool::new("trae", "Trae", ".trae", ".trae/skills"),
        Tool::new("trae-cn", "Trae CN", ".trae-cn", ".trae-cn/skills"),
        // W
        Tool::new(
            "windsurf",
            "Windsurf",
            ".codeium/windsurf",
            ".codeium/windsurf/skills",
        ),
        // Z
        Tool::new("zencoder", "Zencoder", ".zencoder", ".zencoder/skills"),
    ]
}

/// Get a specific tool by name
pub fn get_tool(name: &str) -> Option<Tool> {
    all_tools().into_iter().find(|t| t.name == name)
}

/// Get default source path for skills
pub fn default_source_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("~"))
        .join("Dev/scripts/skills/skills")
}
