use std::path::{Path, PathBuf};

pub struct ToolDetector;

impl ToolDetector {
    pub fn detect_all() -> Vec<String> {
        let mut detected = Vec::new();

        if Self::detect_opencode() {
            detected.push("opencode".to_string());
        }
        if Self::detect_claude() {
            detected.push("claude".to_string());
        }
        if Self::detect_codex() {
            detected.push("codex".to_string());
        }
        if Self::detect_cursor() {
            detected.push("cursor".to_string());
        }
        if Self::detect_amp() {
            detected.push("amp".to_string());
        }
        if Self::detect_antigravity() {
            detected.push("antigravity".to_string());
        }

        detected
    }

    fn detect_opencode() -> bool {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Path::new(&home.join(".config/opencode")).exists()
    }

    fn detect_claude() -> bool {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Path::new(&home.join(".claude")).exists()
    }

    fn detect_codex() -> bool {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Path::new(&home.join(".codex")).exists()
    }

    fn detect_cursor() -> bool {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Path::new(&home.join(".cursor")).exists()
    }

    fn detect_amp() -> bool {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Path::new(&home.join(".agents")).exists()
    }

    fn detect_antigravity() -> bool {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));
        Path::new(&home.join(".agent")).exists()
    }
}
