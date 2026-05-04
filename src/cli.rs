use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Multi-Agent Consensus Engine (MACE)")]
pub struct Cli {
    /// Optional workspace path. If omitted, uses current directory.
    pub path: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize MACE configuration (config.toml) in user's XDG config directory
    Init,
    /// Configure roles and agents interactively
    Config {
        /// The role to configure (optional)
        role: Option<String>,
    },
    /// Dispatch a concurrent task
    Run {
        /// The prompt/task to execute
        prompt: String,
        /// Comma-separated list of roles to use
        #[arg(short, long)]
        roles: Option<String>,
        /// Comma-separated list of models to use (legacy)
        #[arg(short, long)]
        models: Option<String>,
    },
    /// Audit mode: Force MACE to cross-review the current git staging area
    Audit {
        #[arg(short, long, default_value = "cross-review")]
        strategy: String,
    },
    /// Resume an interrupted task or HITL deadlock
    Resume {
        task_id: String,
    },
    /// List, configure, or troubleshoot available tools
    Tools,
    /// Alias for tools
    Doctor,
}
