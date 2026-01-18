use crate::config::{self, Config};
use crate::detect::ToolDetector;
use crate::sync::SyncManager;
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

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
    /// Show sync status
    Status,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init_config(),
        Commands::Config => show_config(),
        Commands::DetectTools => detect_tools(),
        Commands::Sync => sync_skills(),
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

    let default_config = Config::default();
    config::save_config(&default_config).map_err(|e| {
        anyhow!(
            "Failed to save configuration to {}: {}",
            config_path.display(),
            e
        )
    })?;

    println!("Configuration created at: {}", config_path.display());
    println!("You can now edit the file to customize your settings.");
    println!("Run 'capsync detect-tools' to auto-detect installed tools.");

    Ok(())
}

fn show_config() -> Result<()> {
    let config = config::load_config()?;
    let config_path = config::get_config_path();

    println!("Current Configuration:");
    println!("=====================");
    println!("Config file: {}", config_path.display());
    println!("Source Directory: {}", config.source.directory.display());
    println!();

    println!("Tools:");
    println!(
        "  opencode: enabled={}, path={}",
        config.tools.opencode.enabled,
        config.tools.opencode.path.display()
    );
    println!(
        "  claude: enabled={}, path={}",
        config.tools.claude.enabled,
        config.tools.claude.path.display()
    );
    println!(
        "  codex: enabled={}, path={}",
        config.tools.codex.enabled,
        config.tools.codex.path.display()
    );
    println!(
        "  cursor: enabled={}, path={}",
        config.tools.cursor.enabled,
        config.tools.cursor.path.display()
    );
    println!(
        "  amp: enabled={}, path={}",
        config.tools.amp.enabled,
        config.tools.amp.path.display()
    );
    println!(
        "  antigravity: enabled={}, path={}",
        config.tools.antigravity.enabled,
        config.tools.antigravity.path.display()
    );
    println!();

    println!("Sync Settings:");
    println!("  auto_detect: {}", config.sync.auto_detect);
    println!("  create_dirs: {}", config.sync.create_dirs);

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

    let result = SyncManager::sync_all(&config)?;
    result.print_summary();

    Ok(())
}

fn show_status() -> Result<()> {
    let config = config::load_config()?;

    println!("Sync Status:");
    println!("============");

    if !config.source.directory.exists() {
        println!(
            "Source directory does not exist: {}",
            config.source.directory.display()
        );
        return Ok(());
    }

    println!("Source directory: {}", config.source.directory.display());

    let skills = std::fs::read_dir(&config.source.directory)
        .map_err(|e| anyhow!("Failed to read source directory: {}", e))?;
    let skill_count = skills.count();
    println!("Skills found: {}", skill_count);

    println!("\nEnabled Tools:");

    if config.tools.opencode.enabled {
        println!("  opencode: {}", config.tools.opencode.path.display());
    }

    if config.tools.claude.enabled {
        println!("  claude: {}", config.tools.claude.path.display());
    }

    if config.tools.codex.enabled {
        println!("  codex: {}", config.tools.codex.path.display());
    }

    if config.tools.cursor.enabled {
        println!("  cursor: {}", config.tools.cursor.path.display());
    }

    if config.tools.amp.enabled {
        println!("  amp: {}", config.tools.amp.path.display());
    }

    if config.tools.antigravity.enabled {
        println!("  antigravity: {}", config.tools.antigravity.path.display());
    }

    Ok(())
}
