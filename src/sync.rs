//! Synchronization engine for CapSync
//!
//! This module provides:
//! - Skill directory scanning
//! - Symlink creation and management
//! - Cleanup of obsolete symlinks
//! - Sync result reporting

use crate::config::{Config, ToolConfig};
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct SyncManager;

impl SyncManager {
    pub fn sync_all(config: &Config) -> Result<SyncResult> {
        let mut result = SyncResult::new();

        if !config.source.directory.exists() {
            return Err(anyhow!(
                "Source directory does not exist: {}",
                config.source.directory.display()
            ));
        }

        let skills = Self::get_skills(&config.source.directory)?;

        if config.tools.opencode.enabled {
            result.add_tool_result(
                "opencode",
                Self::sync_tool(&config.tools.opencode, &skills, &config.source.directory)?,
            );
        }

        if config.tools.claude.enabled {
            result.add_tool_result(
                "claude",
                Self::sync_tool(&config.tools.claude, &skills, &config.source.directory)?,
            );
        }

        if config.tools.codex.enabled {
            result.add_tool_result(
                "codex",
                Self::sync_tool(&config.tools.codex, &skills, &config.source.directory)?,
            );
        }

        if config.tools.cursor.enabled {
            result.add_tool_result(
                "cursor",
                Self::sync_tool(&config.tools.cursor, &skills, &config.source.directory)?,
            );
        }

        if config.tools.amp.enabled {
            result.add_tool_result(
                "amp",
                Self::sync_tool(&config.tools.amp, &skills, &config.source.directory)?,
            );
        }

        if config.tools.antigravity.enabled {
            result.add_tool_result(
                "antigravity",
                Self::sync_tool(&config.tools.antigravity, &skills, &config.source.directory)?,
            );
        }

        Ok(result)
    }

    fn get_skills(source_dir: &Path) -> Result<Vec<String>> {
        let mut skills = Vec::new();

        for entry in fs::read_dir(source_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if let Some(skill_name) = path.file_name().and_then(|n| n.to_str()) {
                    // Skip hidden directories and common non-skill directories
                    if !skill_name.starts_with('.')
                        && skill_name != "docs"
                        && skill_name != "scripts"
                        && skill_name != "commands"
                    {
                        skills.push(skill_name.to_string());
                    }
                }
            }
        }

        Ok(skills)
    }

    fn sync_tool(
        tool_config: &ToolConfig,
        skills: &[String],
        source_dir: &Path,
    ) -> Result<ToolSyncResult> {
        let mut tool_result = ToolSyncResult::new(tool_config.path.clone());

        // Create destination directory if it doesn't exist
        if !tool_config.path.exists() {
            fs::create_dir_all(&tool_config.path)?;
            tool_result.add_message("Created destination directory".to_string());
        }

        // Remove existing symlinks that don't correspond to current skills
        Self::cleanup_old_symlinks(&tool_config.path, skills, &mut tool_result)?;

        // Create/update symlinks for current skills
        for skill in skills {
            let source_path = source_dir.join(skill);
            let dest_path = tool_config.path.join(skill);

            match Self::create_symlink(&source_path, &dest_path) {
                Ok(_) => tool_result.add_success(skill),
                Err(e) => tool_result.add_error(skill, e.to_string()),
            }
        }

        Ok(tool_result)
    }

    fn cleanup_old_symlinks(
        dest_dir: &Path,
        current_skills: &[String],
        result: &mut ToolSyncResult,
    ) -> Result<()> {
        if !dest_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dest_dir).with_context(|| {
            format!(
                "Failed to read destination directory: {}",
                dest_dir.display()
            )
        })? {
            let entry = entry?;
            let path = entry.path();

            if path.is_symlink() {
                if let Some(skill_name) = path.file_name().and_then(|n| n.to_str()) {
                    if !current_skills.contains(&skill_name.to_string()) {
                        fs::remove_file(&path).with_context(|| {
                            format!("Failed to remove old symlink: {}", path.display())
                        })?;
                        result.add_message(format!("Removed old symlink: {}", skill_name));
                    }
                }
            }
        }

        Ok(())
    }

    fn create_symlink(source: &Path, dest: &Path) -> Result<()> {
        // Remove existing symlink/file if it exists
        if dest.exists() || dest.is_symlink() {
            fs::remove_file(dest)?;
        }

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(source, dest).with_context(|| {
                format!(
                    "Failed to create symlink from {} to {}",
                    source.display(),
                    dest.display()
                )
            })?;
        }

        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_dir(source, dest).with_context(|| {
                format!(
                    "Failed to create symlink from {} to {}",
                    source.display(),
                    dest.display()
                )
            })?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct SyncResult {
    pub tools: Vec<(String, ToolSyncResult)>,
}

impl Default for SyncResult {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncResult {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn add_tool_result(&mut self, tool_name: &str, result: ToolSyncResult) {
        self.tools.push((tool_name.to_string(), result));
    }

    pub fn print_summary(&self) {
        println!("Sync Results:");
        println!("=============");

        for (tool_name, result) in &self.tools {
            println!("\n{}:", tool_name);
            result.print();
        }
    }
}

/// Result of syncing skills to a single tool
#[derive(Debug)]
pub struct ToolSyncResult {
    /// Destination path for this tool
    pub path: PathBuf,
    /// Skills successfully synced
    pub successful: Vec<String>,
    /// Skills that failed to sync with error messages
    pub errors: Vec<(String, String)>,
    /// Informational messages (created directories, cleaned up symlinks, etc.)
    pub messages: Vec<String>,
}

impl ToolSyncResult {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            successful: Vec::new(),
            errors: Vec::new(),
            messages: Vec::new(),
        }
    }

    pub fn add_success(&mut self, skill: &str) {
        self.successful.push(skill.to_string());
    }

    pub fn add_error(&mut self, skill: &str, error: String) {
        self.errors.push((skill.to_string(), error));
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn print(&self) {
        println!("  Path: {}", self.path.display());

        if !self.messages.is_empty() {
            println!("  Messages:");
            for msg in &self.messages {
                println!("    {}", msg);
            }
        }

        if !self.successful.is_empty() {
            println!("  Successful: {}", self.successful.len());
            for skill in &self.successful {
                println!("    ✓ {}", skill);
            }
        }

        if !self.errors.is_empty() {
            println!("  Errors: {}", self.errors.len());
            for (skill, error) in &self.errors {
                println!("    ✗ {}: {}", skill, error);
            }
        }
    }
}
