use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MaceConfig {
    #[serde(default)]
    pub agents: HashMap<String, AgentDef>,
    #[serde(default)]
    pub roles: HashMap<String, RoleDef>,
    #[serde(default)]
    pub consensus: ConsensusConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentDef {
    pub cmd: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RoleDef {
    pub agent: String, // References a key in the agents map
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

/// Detects which AI CLI tools are installed in the system PATH
pub fn detect_tools() -> Vec<String> {
    let candidates = vec!["claude", "gemini", "aider", "gh", "copilot", "copilot-cli", "codex"];
    candidates.into_iter()
        .filter(|&tool| {
            let cmd = if cfg!(target_os = "windows") {
                format!("{}.cmd", tool)
            } else {
                tool.to_string()
            };
            
            // Just check if the executable exists in PATH
            #[cfg(not(target_os = "windows"))]
            {
                std::process::Command::new("which")
                    .arg(&cmd)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("where")
                    .arg(&cmd)
                    .stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null())
                    .status()
                    .map(|s| s.success())
                    .unwrap_or(false)
            }
        })
        .map(|s| s.to_string())
        .collect()
}

pub fn generate_default_config(path: &Path) -> Result<()> {
    // Try to load existing config, or start with default
    let mut config = if path.exists() {
        let toml_str = fs::read_to_string(path)?;
        toml::from_str(&toml_str).unwrap_or_default()
    } else {
        MaceConfig::default()
    };

    let detected = detect_tools();
    if detected.is_empty() && config.agents.is_empty() {
        println!("⚠️  No AI CLI tools detected. Adding a skeleton agent.");
        config.agents.insert("aider".to_string(), AgentDef { cmd: "aider".to_string() });
        config.roles.insert("coder".to_string(), RoleDef {
            agent: "aider".to_string(),
            model: None,
            prompt_template: Some("implementer.md".to_string()),
            context_window: None,
        });
    } else {
        println!("🔍 Scanning for agents...");
        for tool in detected {
            let agent_key = match tool.as_str() {
                "gh" => "github-copilot".to_string(),
                _ => tool.clone(),
            };

            // 1. Add to agents list if not already there
            if !config.agents.contains_key(&agent_key) {
                println!("✨ New agent detected: {}", agent_key);
                let cmd = match tool.as_str() {
                    "gh" => "gh copilot".to_string(),
                    _ => tool.clone(),
                };
                config.agents.insert(agent_key.clone(), AgentDef { cmd });

                // 2. Assign a default role for this new agent if the role doesn't exist
                match agent_key.as_str() {
                    "claude" => {
                        config.roles.entry("architect".to_string()).or_insert(RoleDef {
                            agent: "claude".to_string(),
                            model: None,
                            prompt_template: Some("architect.md".to_string()),
                            context_window: None,
                        });
                    },
                    "gemini" => {
                        config.roles.entry("reviewer".to_string()).or_insert(RoleDef {
                            agent: "gemini".to_string(),
                            model: None,
                            prompt_template: None,
                            context_window: Some("1M".to_string()),
                        });
                    },
                    "aider" => {
                        config.roles.entry("coder".to_string()).or_insert(RoleDef {
                            agent: "aider".to_string(),
                            model: None,
                            prompt_template: None,
                            context_window: None,
                        });
                    },
                    "github-copilot" => {
                        config.roles.entry("navigator".to_string()).or_insert(RoleDef {
                            agent: "github-copilot".to_string(),
                            model: None,
                            prompt_template: None,
                            context_window: None,
                        });
                    },
                    "copilot" | "copilot-cli" => {
                        config.roles.entry("helper".to_string()).or_insert(RoleDef {
                            agent: agent_key.clone(),
                            model: None,
                            prompt_template: None,
                            context_window: None,
                        });
                    },
                    "codex" => {
                        config.roles.entry("researcher".to_string()).or_insert(RoleDef {
                            agent: "codex".to_string(),
                            model: None,
                            prompt_template: None,
                            context_window: None,
                        });
                    },
                    _ => {}
                }
            }
        }
    }

    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(path, toml_str).context("Failed to write config file")?;
    Ok(())
}

use dialoguer::{theme::ColorfulTheme, Select, Input};

pub fn run_interactive_config(path: &Path, target_role: Option<String>) -> Result<()> {
    let mut config = load_config(path)?;
    
    if config.agents.is_empty() {
        println!("❌ No agents found. Please run `mace init` first or manually add agents to the config file.");
        return Ok(());
    }

    let roles_list: Vec<String> = config.roles.keys().cloned().collect();
    
    let role_name = if let Some(r) = target_role {
        r
    } else {
        if roles_list.is_empty() {
            Input::<String>::with_theme(&ColorfulTheme::default())
                .with_prompt("No roles defined. Enter a name for a new role")
                .interact_text()?
        } else {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a role to configure")
                .items(&roles_list)
                .default(0)
                .interact_opt()?;
            
            match selection {
                Some(index) => roles_list[index].clone(),
                None => return Ok(()),
            }
        }
    };

    let agent_keys: Vec<String> = config.agents.keys().cloned().collect();
    let mut options = agent_keys.clone();
    options.push("[Add New Agent]".to_string());

    let agent_selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Select an agent for role '{}'", role_name))
        .items(&options)
        .default(0)
        .interact()?;

    let selected_agent = if agent_selection < agent_keys.len() {
        agent_keys[agent_selection].clone()
    } else {
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter name for the new agent")
            .interact_text()?;
        let cmd: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Enter command for agent '{}'", name))
            .interact_text()?;
        
        config.agents.insert(name.clone(), AgentDef { cmd });
        name
    };

    // Update the role
    let mut role_def = config.roles.get(&role_name).cloned().unwrap_or(RoleDef {
        agent: selected_agent.clone(),
        model: None,
        prompt_template: None,
        context_window: None,
    });
    role_def.agent = selected_agent;
    
    config.roles.insert(role_name.clone(), role_def);

    // Save changes
    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(path, toml_str).context("Failed to save configuration")?;
    
    println!("✅ Role '{}' updated to use agent '{}'.", role_name, config.roles.get(&role_name).unwrap().agent);
    
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
