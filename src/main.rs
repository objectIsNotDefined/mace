mod cli;
mod config;
mod tui;

use clap::Parser;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    // Force global config path: ~/.config/mace/config.toml (or equivalent on Windows)
    #[cfg(target_os = "windows")]
    let config_dir = dirs::config_dir()
        .context("Unable to locate user config directory")?
        .join("mace");
    
    #[cfg(not(target_os = "windows"))]
    let config_dir = dirs::home_dir()
        .context("Unable to locate user home directory")?
        .join(".config")
        .join("mace");

    let config_path = config_dir.join("config.toml");

    if cli.command.is_none() {
        // Run TUI mode
        if let Err(e) = tui::run_tui().await {
            eprintln!("TUI Error: {}", e);
        }
        return Ok(());
    }

    match cli.command.unwrap() {
        cli::Commands::Init => {
            println!("🚀 Initializing global MACE configuration...");
            // Ensure the global config directory exists
            if !config_dir.exists() {
                std::fs::create_dir_all(&config_dir)
                    .context("Failed to create global config directory")?;
            }
            
            match config::generate_default_config(&config_path) {
                Ok(_) => println!("✅ Successfully generated global configuration at {:?}", config_path),
                Err(e) => {
                    eprintln!("❌ Failed to initialize config: {}", e);
                    std::process::exit(1);
                }
            }
        }
        cli::Commands::Config { role } => {
            config::run_interactive_config(&config_path, role.clone())?;
        }
        cli::Commands::Run { prompt, roles, models: _ } => {
            let config = config::load_config(&config_path)?;
            println!("⚙️  Global configuration loaded from {:?}", config_path);
            println!("🤖 Roles available: {}", config.roles.keys().cloned().collect::<Vec<_>>().join(", "));
            println!("📝 Task: {}", prompt);
            if let Some(r) = roles {
                println!("🎭 Using roles: {}", r);
            }
            // TODO: Execute task
        }
        cli::Commands::Audit { strategy } => {
            println!("Audit mode using strategy: {}", strategy);
            // TODO: Execute audit
        }
        cli::Commands::Resume { task_id } => {
            println!("Resuming task: {}", task_id);
            // TODO: Execute resume
        }
        cli::Commands::Tools | cli::Commands::Doctor => {
            println!("Checking available AI tools...");
            // TODO: Implement tool diagnostics
        }
    }

    Ok(())
}
