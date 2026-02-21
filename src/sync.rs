use crate::config::Config;
use anyhow::{anyhow, Context, Result};
use std::fs;
use std::path::Path;

pub struct SyncManager;

impl SyncManager {
    pub fn sync_skills(config: &Config) -> Result<SyncResult> {
        let mut result = SyncResult::new();

        if !config.skills_source.exists() {
            return Err(anyhow!(
                "Skills source directory does not exist: {}",
                config.skills_source.display()
            ));
        }

        for (name, dest_config) in &config.destinations {
            if dest_config.enabled {
                match Self::sync_destination(&config.skills_source, &dest_config.skills_path) {
                    Ok(()) => result.add_success(name),
                    Err(e) => result.add_error(name, e.to_string()),
                }
            }
        }

        Ok(result)
    }

    pub fn sync_commands(config: &Config) -> Result<SyncResult> {
        let mut result = SyncResult::new();

        let commands_source = match &config.commands_source {
            Some(source) if !source.as_os_str().is_empty() => source,
            _ => return Ok(result),
        };

        if !commands_source.exists() {
            return Err(anyhow!(
                "Commands source directory does not exist: {}",
                commands_source.display()
            ));
        }

        for (name, dest_config) in &config.destinations {
            if dest_config.enabled {
                if let Some(commands_path) = &dest_config.commands_path {
                    match Self::sync_destination(commands_source, commands_path) {
                        Ok(()) => result.add_success(&format!("{name} (commands)")),
                        Err(e) => result.add_error(&format!("{name} (commands)"), e.to_string()),
                    }
                }
            }
        }

        Ok(result)
    }

    pub fn sync_all(config: &Config) -> Result<SyncResult> {
        let mut result = SyncResult::new();

        let skills_result = Self::sync_skills(config)?;
        result.merge(skills_result);

        if config.has_commands() {
            let commands_result = Self::sync_commands(config)?;
            result.merge(commands_result);
        }

        Ok(result)
    }

    fn sync_destination(source: &Path, dest: &Path) -> Result<()> {
        if let Some(parent) = dest.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        if dest.exists() || dest.is_symlink() {
            if dest.is_symlink() {
                fs::remove_file(dest)?;
            } else if dest.is_dir() {
                fs::remove_dir_all(dest)?;
            } else {
                fs::remove_file(dest)?;
            }
        }

        Self::create_symlink(source, dest)?;

        Ok(())
    }

    pub fn remove(name: &str, config: &Config) -> Result<()> {
        if let Some(dest_config) = config.destinations.get(name) {
            let dest = &dest_config.skills_path;
            Self::remove_symlink(dest, name)?;

            if let Some(commands_dest) = &dest_config.commands_path {
                if commands_dest.is_symlink() || commands_dest.exists() {
                    Self::remove_symlink(commands_dest, &format!("{} (commands)", name))?;
                }
            }

            Ok(())
        } else {
            Err(anyhow!("Unknown destination: {}", name))
        }
    }

    fn remove_symlink(dest: &Path, name: &str) -> Result<()> {
        if dest.is_symlink() {
            fs::remove_file(dest)
                .with_context(|| format!("Failed to remove symlink at {}", dest.display()))?;
            println!("Removed {} symlink from {}", name, dest.display());
            Ok(())
        } else if dest.exists() {
            Err(anyhow!(
                "Destination at {} exists but is not a symlink. Not removing.",
                dest.display()
            ))
        } else {
            println!(
                "No {} symlink found at {} (already removed?)",
                name,
                dest.display()
            );
            Ok(())
        }
    }

    pub fn remove_all(config: &Config) -> Result<()> {
        for (name, dest_config) in &config.destinations {
            let dest = &dest_config.skills_path;
            if dest.is_symlink() || dest.exists() {
                match Self::remove_symlink(dest, name) {
                    Ok(()) => {}
                    Err(e) => eprintln!("Failed to remove symlink from {}: {e}", dest.display()),
                }
            }

            if let Some(commands_dest) = &dest_config.commands_path {
                if commands_dest.is_symlink() || commands_dest.exists() {
                    match Self::remove_symlink(commands_dest, &format!("{} (commands)", name)) {
                        Ok(()) => {}
                        Err(e) => eprintln!("Failed to remove commands symlink: {e}"),
                    }
                }
            }
        }
        Ok(())
    }

    fn create_symlink(source: &Path, dest: &Path) -> Result<()> {
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
    pub successful: Vec<String>,
    pub errors: Vec<(String, String)>,
}

impl Default for SyncResult {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncResult {
    pub fn new() -> Self {
        Self {
            successful: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_success(&mut self, name: &str) {
        self.successful.push(name.to_string());
    }

    pub fn add_error(&mut self, name: &str, error: String) {
        self.errors.push((name.to_string(), error));
    }

    pub fn merge(&mut self, other: SyncResult) {
        self.successful.extend(other.successful);
        self.errors.extend(other.errors);
    }

    pub fn print(&self) {
        if !self.successful.is_empty() {
            println!("\nSynced successfully:");
            for name in &self.successful {
                println!("  {name}");
            }
        }

        if !self.errors.is_empty() {
            println!("\nErrors:");
            for (name, error) in &self.errors {
                println!("  {name}: {error}");
            }
        }
    }
}
