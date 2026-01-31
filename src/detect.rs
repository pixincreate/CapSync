use crate::tools::all_tools;

pub struct ToolDetector;

impl ToolDetector {
    pub fn detect_all() -> Vec<String> {
        let mut detected = Vec::new();

        for tool in all_tools() {
            if tool.config_path.exists() {
                detected.push(tool.name.to_string());
            }
        }

        detected
    }
}
