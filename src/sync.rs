use crate::config::{Config, DestinationConfig};
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::Path;

pub struct SyncManager;

impl SyncManager {
    pub fn sync_all(config: &Config) -> Result<SyncResult> {
        let mut result = SyncResult::new();

        if !config.source.exists() {
            return Err(anyhow!(
                "Source directory does not exist: {}",
                config.source.display()
            ));
        }

        for (name, dest_config) in &config.destinations {
            if dest_config.enabled {
                match Self::sync_destination(&config.source, dest_config) {
                    Ok(()) => result.add_success(name),
                    Err(e) => result.add_error(name, e.to_string()),
                }
            }
        }

        Ok(result)
    }

    fn sync_destination(source: &Path, dest_config: &DestinationConfig) -> Result<()> {
        let dest = &dest_config.path;

        // Create parent directory if it doesn't exist
        if let Some(parent) = dest.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }

        // Remove existing symlink/file/directory if it exists
        if dest.exists() || dest.is_symlink() {
            if dest.is_symlink() {
                fs::remove_file(dest)?;
            } else if dest.is_dir() {
                fs::remove_dir_all(dest)?;
            } else {
                fs::remove_file(dest)?;
            }
        }

        // Create symlink
        Self::create_symlink(source, dest)?;

        Ok(())
    }

    pub fn remove(name: &str, config: &Config) -> Result<()> {
        if let Some(dest_config) = config.destinations.get(name) {
            let dest = &dest_config.path;
            if dest.is_symlink() {
                fs::remove_file(dest)
                    .with_context(|| format!("Failed to remove symlink at {}", dest.display()))?;
                println!("Removed symlink from {}", dest.display());
                Ok(())
            } else if dest.exists() {
                Err(anyhow!(
                    "Destination at {} exists but is not a symlink. Not removing.",
                    dest.display()
                ))
            } else {
                println!("No symlink found at {} (already removed?)", dest.display());
                Ok(())
            }
        } else {
            Err(anyhow!("Unknown destination: {}", name))
        }
    }

    pub fn remove_all(config: &Config) -> Result<()> {
        for (name, dest_config) in &config.destinations {
            let dest = &dest_config.path;
            if dest.is_symlink() {
                match fs::remove_file(dest) {
                    Ok(()) => println!("Removed symlink from {} ({name})", dest.display()),
                    Err(e) => eprintln!("Failed to remove symlink from {}: {e}", dest.display()),
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

    pub fn print(&self) {
        if !self.successful.is_empty() {
            println!("\nSynced successfully:");
            for name in &self.successful {
                println!("  ✓ {name}");
            }
        }

        if !self.errors.is_empty() {
            println!("\nErrors:");
            for (name, error) in &self.errors {
                println!("  ✗ {name}: {error}");
            }
        }
    }
}
