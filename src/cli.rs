use crate::config::{self, Config};
use crate::detect::ToolDetector;
use crate::sync::SyncManager;
use anyhow::Result;
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
    config::save_config(&default_config)?;

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
            println!("✓ {}", tool);
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
            "❌ Source directory does not exist: {}",
            config.source.directory.display()
        );
        return Ok(());
    }

    println!("✅ Source directory: {}", config.source.directory.display());

    let skills = std::fs::read_dir(&config.source.directory)?;
    let skill_count = skills.count();
    println!("📁 Skills found: {}", skill_count);

    println!("\nEnabled Tools:");

    if config.tools.opencode.enabled {
        let status = if config.tools.opencode.path.exists() {
            "✅"
        } else {
            "❌"
        };
        println!(
            "  {} opencode: {}",
            status,
            config.tools.opencode.path.display()
        );
    }

    if config.tools.claude.enabled {
        let status = if config.tools.claude.path.exists() {
            "✅"
        } else {
            "❌"
        };
        println!(
            "  {} claude: {}",
            status,
            config.tools.claude.path.display()
        );
    }

    if config.tools.codex.enabled {
        let status = if config.tools.codex.path.exists() {
            "✅"
        } else {
            "❌"
        };
        println!("  {} codex: {}", status, config.tools.codex.path.display());
    }

    if config.tools.cursor.enabled {
        let status = if config.tools.cursor.path.exists() {
            "✅"
        } else {
            "❌"
        };
        println!(
            "  {} cursor: {}",
            status,
            config.tools.cursor.path.display()
        );
    }

    if config.tools.amp.enabled {
        let status = if config.tools.amp.path.exists() {
            "✅"
        } else {
            "❌"
        };
        println!("  {} amp: {}", status, config.tools.amp.path.display());
    }

    if config.tools.antigravity.enabled {
        let status = if config.tools.antigravity.path.exists() {
            "✅"
        } else {
            "❌"
        };
        println!(
            "  {} antigravity: {}",
            status,
            config.tools.antigravity.path.display()
        );
    }

    Ok(())
}
