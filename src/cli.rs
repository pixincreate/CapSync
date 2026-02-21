use crate::config::{self, Config, DestinationConfig};
use crate::detect::ToolDetector;
use crate::sync::SyncManager;
use crate::tools::{all_tools, get_tool};
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
    /// Add a tool to configuration and sync
    Add {
        /// Tool name to add
        tool: String,
        /// Skip syncing after adding
        #[arg(long)]
        no_sync: bool,
    },
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
        Commands::Sync => sync_all(),
        Commands::Add { tool, no_sync } => add_tool(&tool, no_sync),
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
        print!("Re-initialize? This will overwrite your current config. [y/N]: ");
        io::stdout().flush()?;
        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm)?;
        if confirm.trim().to_lowercase() != "y" {
            println!("Aborted.");
            return Ok(());
        }
    }

    println!("Welcome to CapSync! Let's set up your configuration.\n");

    let skills_source = loop {
        print!("Enter your skills source directory: ");
        io::stdout().flush()?;
        let mut source_input = String::new();
        io::stdin().read_line(&mut source_input)?;
        let trimmed = source_input.trim();
        if !trimmed.is_empty() {
            let expanded =
                shellexpand::full(trimmed).map_err(|e| anyhow!("Failed to expand path: {}", e))?;
            break PathBuf::from(expanded.as_ref());
        }
        println!("Please enter a path.");
    };

    let commands_source = {
        let commands_dir = skills_source.join("commands");
        if commands_dir.exists() && commands_dir.is_dir() {
            println!("\nFound commands/ subdirectory in skills source.");
            loop {
                print!("Enable commands? [Y/n]: ");
                io::stdout().flush()?;
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let trimmed = input.trim().to_lowercase();
                if trimmed.is_empty() || trimmed == "y" {
                    break Some(commands_dir);
                } else if trimmed == "n" {
                    break None;
                }
                println!("Please enter Y or n.");
            }
        } else {
            println!("\n(Optional) Enter commands source directory");
            print!("(or press Enter to skip): ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let trimmed = input.trim();
            if trimmed.is_empty() {
                None
            } else {
                let expanded = shellexpand::full(trimmed)
                    .map_err(|e| anyhow!("Failed to expand path: {}", e))?;
                Some(PathBuf::from(expanded.as_ref()))
            }
        }
    };

    println!("\nDetecting installed tools...");
    let detected = ToolDetector::detect_all();

    let mut destinations = HashMap::new();

    // Only add detected tools to config
    for tool in all_tools() {
        if detected.contains(&tool.name.to_string()) {
            destinations.insert(
                tool.name.to_string(),
                DestinationConfig {
                    enabled: true,
                    skills_path: tool.skills_path.clone(),
                    commands_path: tool.commands_path.clone(),
                },
            );
        }
    }

    if detected.is_empty() {
        println!("No tools detected. You can manually add tools to the config later.");
    } else {
        println!("Detected and enabled: {}", detected.join(", "));
    }

    // Create and save config
    let config = Config {
        skills_source,
        commands_source,
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
    println!("  - Run 'capsync sync' to sync your skills and commands");
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
    println!("Skills source: {}", config.skills_source.display());

    if let Some(commands_source) = &config.commands_source {
        println!("Commands source: {}", commands_source.display());
    } else {
        println!("Commands source: (not configured)");
    }

    println!();

    let enabled: Vec<_> = config
        .destinations
        .iter()
        .filter(|(_, dest)| dest.enabled)
        .collect();

    if enabled.is_empty() {
        println!("No tools enabled.");
    } else {
        println!("Enabled tools:");
        for (name, dest) in enabled {
            println!("  {}: {}", name, dest.skills_path.display());
            if let Some(commands_path) = &dest.commands_path {
                println!("  {} commands: {}", name, commands_path.display());
            }
        }
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

fn sync_all() -> Result<()> {
    let config = config::load_config()?;

    println!("Syncing skills and commands...");
    println!("===========================");
    println!("Skills source: {}", config.skills_source.display());

    if let Some(commands_source) = &config.commands_source {
        println!("Commands source: {}", commands_source.display());
    }

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

fn add_tool(tool_name: &str, no_sync: bool) -> Result<()> {
    let mut config = config::load_config()?;

    // Validate tool exists
    let tool = get_tool(tool_name).ok_or_else(|| {
        anyhow!(
            "Tool '{}' does not exist or is unsupported in the current version",
            tool_name
        )
    })?;

    // Check if already in config
    if config.destinations.contains_key(tool_name) {
        println!("Tool '{}' is already in the configuration", tool_name);
        if !no_sync {
            println!("Running sync...");
            return sync_all();
        }
        return Ok(());
    }

    // Add tool to config
    config.destinations.insert(
        tool_name.to_string(),
        DestinationConfig {
            enabled: true,
            skills_path: tool.skills_path.clone(),
            commands_path: tool.commands_path.clone(),
        },
    );

    config::save_config(&config)?;
    println!("Added '{}' to configuration", tool_name);

    if !no_sync {
        println!("Running sync...");
        sync_all()
    } else {
        Ok(())
    }
}

fn show_status() -> Result<()> {
    let config = config::load_config()?;

    println!("Status:");
    println!("=======");

    if config.skills_source.exists() {
        println!("Skills source: {}", config.skills_source.display());
    } else {
        println!(
            "Skills source: {} (does not exist)",
            config.skills_source.display()
        );
    }

    if let Some(commands_source) = &config.commands_source {
        if commands_source.exists() {
            println!("Commands source: {}", commands_source.display());
        } else {
            println!(
                "Commands source: {} (does not exist)",
                commands_source.display()
            );
        }
    } else {
        println!("Commands source: (not configured)");
    }

    println!("\nDestinations:");
    for (name, dest) in &config.destinations {
        let path = &dest.skills_path;
        if path.is_symlink() {
            match path.read_link() {
                Ok(target) => {
                    if target.exists() {
                        println!(
                            "  {}: {} (symlink -> {})",
                            name,
                            path.display(),
                            target.display()
                        );
                    } else {
                        println!(
                            "  {}: {} (broken symlink -> {})",
                            name,
                            path.display(),
                            target.display()
                        );
                    }
                }
                Err(_) => {
                    println!("  {}: {} (cannot read symlink)", name, path.display());
                }
            }
        } else if path.exists() {
            println!("  {}: {} (exists, not a symlink)", name, path.display());
        } else {
            println!("  {}: {} - (not synced)", name, path.display());
        }

        if let Some(commands_path) = &dest.commands_path {
            if commands_path.is_symlink() {
                match commands_path.read_link() {
                    Ok(target) => {
                        if target.exists() {
                            println!(
                                "    commands: {} (symlink -> {})",
                                commands_path.display(),
                                target.display()
                            );
                        } else {
                            println!(
                                "    commands: {} (broken symlink -> {})",
                                commands_path.display(),
                                target.display()
                            );
                        }
                    }
                    Err(_) => {
                        println!(
                            "    commands: {} (cannot read symlink)",
                            commands_path.display()
                        );
                    }
                }
            } else if commands_path.exists() {
                println!(
                    "    commands: {} (exists, not a symlink)",
                    commands_path.display()
                );
            } else {
                println!("    commands: {} - (not synced)", commands_path.display());
            }
        }
    }

    Ok(())
}
