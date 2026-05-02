use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MaceConfig {
    #[serde(default)]
    pub roles: HashMap<String, RoleDef>,
    #[serde(default)]
    pub consensus: ConsensusConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoleDef {
    pub agent: String,
    pub model: String,
    pub prompt_template: Option<String>,
    pub context_window: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusConfig {
    pub strategy: String,
    pub referee: Option<String>,
    pub max_rounds: u32,
    pub semantic_diff: bool,
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            strategy: "majority-vote".to_string(),
            referee: None,
            max_rounds: 3,
            semantic_diff: true,
        }
    }
}

pub fn generate_default_config(path: &Path) -> Result<()> {
    if path.exists() {
        anyhow::bail!("Configuration file {:?} already exists.", path);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).context("Failed to create configuration directory")?;
    }

    let mut config = MaceConfig::default();
    
    config.roles.insert("architect".to_string(), RoleDef {
        agent: "claude-code".to_string(),
        model: "claude-3.7-sonnet".to_string(),
        prompt_template: Some("architect.md".to_string()),
        context_window: None,
    });
    config.roles.insert("coder".to_string(), RoleDef {
        agent: "aider".to_string(),
        model: "deepseek-coder".to_string(),
        prompt_template: Some("implementer.md".to_string()),
        context_window: None,
    });
    config.roles.insert("reviewer".to_string(), RoleDef {
        agent: "gemini-cli".to_string(),
        model: "gemini-2.5-pro".to_string(),
        prompt_template: None,
        context_window: Some("large".to_string()),
    });

    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(path, toml_str)?;
    Ok(())
}

pub fn load_config(path: &Path) -> Result<MaceConfig> {
    if !path.exists() {
        anyhow::bail!("Configuration file {:?} not found. Please run `mace init`.", path);
    }
    let toml_str = fs::read_to_string(path).context("Failed to read configuration file")?;
    let config: MaceConfig = toml::from_str(&toml_str).context("Failed to parse configuration file")?;
    Ok(config)
}
