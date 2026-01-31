use crate::config::{self, Config, DestinationConfig};
use crate::detect::ToolDetector;
use crate::sync::SyncManager;
use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "capsync")]
#[command(about = "A tool to sync agentic skills across multiple tools")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize configuration file
    Init,
    /// Show current configuration
    Config,
    /// Auto-detect installed tools
    DetectTools,
    /// Sync skills to all enabled tools
    Sync,
    /// Remove symlink from a tool (use --all to remove all)
    Remove {
        /// Tool name to remove symlink from (optional if --all is used)
        tool: Option<String>,
        /// Remove from all destinations
        #[arg(long)]
        all: bool,
    },
    /// Check symlink status
    Status,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_config(),
        Commands::Config => show_config(),
        Commands::DetectTools => detect_tools(),
        Commands::Sync => sync_skills(),
        Commands::Remove { tool, all } => {
            if all {
                remove_all()
            } else if let Some(tool_name) = tool {
                remove_tool(&tool_name)
            } else {
                Err(anyhow!("Either provide a tool name or use --all flag"))
            }
        }
        Commands::Status => show_status(),
    }
}

fn init_config() -> Result<()> {
    let config_path = config::get_config_path();

    if config_path.exists() {
        println!("Configuration already exists at: {}", config_path.display());
        println!("Use 'capsync config' to view current configuration.");
        return Ok(());
    }

    println!("Welcome to CapSync! Let's set up your configuration.\n");

    // Ask for source directory (required)
    let source = loop {
        print!("Enter your skills directory: ");
        io::stdout().flush()?;
        let mut source_input = String::new();
        io::stdin().read_line(&mut source_input)?;
        let trimmed = source_input.trim();
        if !trimmed.is_empty() {
            break PathBuf::from(trimmed);
        }
        println!("Please enter a path.");
    };

    // Auto-detect tools
    println!("\nDetecting installed tools...");
    let detected = ToolDetector::detect_all();

    let mut destinations = HashMap::new();
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("~"));

    // Define all possible tools and their default paths
    let all_tools = [
        ("opencode", home.join(".config/opencode/skill")),
        ("claude", home.join(".claude/skills")),
        ("codex", home.join(".codex/skills")),
        ("cursor", home.join(".cursor/skills")),
        ("amp", home.join(".agents/skills")),
        ("antigravity", home.join(".agent/skills")),
    ];

    for (name, path) in all_tools {
        let detected_enabled = detected.contains(&name.to_string());
        destinations.insert(
            name.to_string(),
            DestinationConfig {
                enabled: detected_enabled,
                path,
            },
        );
    }

    // Show what was detected
    if detected.is_empty() {
        println!("No tools detected. You can enable them manually in the config.");
    } else {
        println!("Detected and enabled: {}", detected.join(", "));
    }

    // Create and save config
    let config = Config {
        source,
        destinations,
    };

    config::save_config(&config).map_err(|e| {
        anyhow!(
            "Failed to save configuration to {}: {}",
            config_path.display(),
            e
        )
    })?;

    println!("\nConfiguration created at: {}", config_path.display());
    println!("\nYou can now:");
    println!("  - Run 'capsync sync' to sync your skills");
    println!("  - Edit the config to enable/disable tools");
    println!("  - Run 'capsync config' to view your settings");

    Ok(())
}

fn show_config() -> Result<()> {
    let config = config::load_config()?;
    let config_path = config::get_config_path();

    println!("Current Configuration:");
    println!("=====================");
    println!("Config file: {}", config_path.display());
    println!("Source: {}", config.source.display());
    println!();

    println!("Destinations:");
    for (name, dest) in &config.destinations {
        println!(
            "  {}: enabled={}, path={}",
            name,
            dest.enabled,
            dest.path.display()
        );
    }

    Ok(())
}

fn detect_tools() -> Result<()> {
    let detected = ToolDetector::detect_all();

    println!("Detected Tools:");
    println!("===============");

    if detected.is_empty() {
        println!("No supported agentic tools detected.");
    } else {
        for tool in &detected {
            println!("{}", tool);
        }
    }

    println!("\nNote: Run 'capsync init' to create/edit configuration");
    println!("      and enable/disable tools as needed.");

    Ok(())
}

fn sync_skills() -> Result<()> {
    let config = config::load_config()?;

    println!("Syncing skills...");
    println!("================");
    println!("Source: {}", config.source.display());

    let result = SyncManager::sync_all(&config)?;
    result.print();

    Ok(())
}

fn remove_tool(tool: &str) -> Result<()> {
    let config = config::load_config()?;
    SyncManager::remove(tool, &config)
}

fn remove_all() -> Result<()> {
    let config = config::load_config()?;
    println!("Removing all symlinks...");
    SyncManager::remove_all(&config)
}

fn show_status() -> Result<()> {
    let config = config::load_config()?;

    println!("Status:");
    println!("=======");

    // Check source
    if config.source.exists() {
        println!("Source: {} ✓", config.source.display());
    } else {
        println!("Source: {} ✗ (does not exist)", config.source.display());
    }

    println!("\nDestinations:");
    for (name, dest) in &config.destinations {
        let path = &dest.path;
        if path.is_symlink() {
            // Check if symlink is broken
            match path.read_link() {
                Ok(target) => {
                    if target.exists() {
                        println!(
                            "  {}: {} ✓ (symlink -> {})",
                            name,
                            path.display(),
                            target.display()
                        );
                    } else {
                        println!(
                            "  {}: {} ⚠ (broken symlink -> {})",
                            name,
                            path.display(),
                            target.display()
                        );
                    }
                }
                Err(_) => {
                    println!("  {}: {} ✗ (cannot read symlink)", name, path.display());
                }
            }
        } else if path.exists() {
            println!("  {}: {} ⚠ (exists, not a symlink)", name, path.display());
        } else {
            println!("  {}: {} - (not synced)", name, path.display());
        }
    }

    Ok(())
}
