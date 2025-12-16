//! Configuration loader module
//!
//! This module handles loading and managing application configuration.

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::utils::error::{AppError, AppResult};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application settings
    pub app: AppSettings,
    /// Database settings
    pub database: DatabaseSettings,
    /// AI service settings
    pub ai: AiSettings,
    /// CLI tool settings
    pub cli: CliToolSettings,
    /// Workspace settings
    pub workspace: WorkspaceSettings,
}

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Data directory
    pub data_dir: String,
    /// Log level
    pub log_level: String,
    /// Enable debug mode
    pub debug: bool,
}

/// Database settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    /// Database URL
    pub url: String,
    /// Maximum connections
    pub max_connections: u32,
    /// Minimum connections
    pub min_connections: u32,
}

/// AI service settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiSettings {
    /// Default AI model
    pub default_model: String,
    /// API timeout in seconds
    pub api_timeout: u64,
    /// Maximum tokens
    pub max_tokens: u32,
    /// Temperature
    pub temperature: f32,
}

/// CLI tool settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliToolSettings {
    /// Node.js path
    pub nodejs_path: String,
    /// Python path
    pub python_path: String,
    /// Git path
    pub git_path: String,
    /// Default shell
    pub default_shell: String,
}

/// Workspace settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceSettings {
    /// Default workspace name
    pub default_workspace: String,
    /// Auto-save interval in seconds
    pub auto_save_interval: u64,
    /// Enable file watching
    pub enable_file_watching: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppSettings {
                name: "Code AI Assistant".to_string(),
                version: "0.1.0".to_string(),
                data_dir: "data".to_string(),
                log_level: "info".to_string(),
                debug: false,
            },
            database: DatabaseSettings {
                url: "sqlite://data/app.db?mode=rwc".to_string(),
                max_connections: 10,
                min_connections: 1,
            },
            ai: AiSettings {
                default_model: "claude-3-5-sonnet".to_string(),
                api_timeout: 30,
                max_tokens: 4096,
                temperature: 0.7,
            },
            cli: CliToolSettings {
                nodejs_path: "node".to_string(),
                python_path: "python".to_string(),
                git_path: "git".to_string(),
                default_shell: "bash".to_string(),
            },
            workspace: WorkspaceSettings {
                default_workspace: "default".to_string(),
                auto_save_interval: 30,
                enable_file_watching: true,
            },
        }
    }
}

/// Load application configuration
pub fn load_config() -> AppResult<AppConfig> {
    info!("Loading application configuration...");

    // Try to load configuration from file
    let config = match load_config_from_file() {
        Ok(config) => {
            info!("Configuration loaded from file");
            config
        }
        Err(_) => {
            info!("No configuration file found, using defaults");
            AppConfig::default()
        }
    };

    info!("Configuration loaded successfully");
    Ok(config)
}

/// Load configuration from file
fn load_config_from_file() -> AppResult<AppConfig> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::ConfigError("Failed to get config directory".to_string()))?
        .join("code-ai-assistant");

    let config_file = config_dir.join("config.toml");

    if !config_file.exists() {
        return Err(AppError::ConfigError("Configuration file not found".to_string()));
    }

    let config_str = std::fs::read_to_string(&config_file)
        .map_err(|e| AppError::ConfigError(format!("Failed to read config file: {}", e)))?;

    let config: AppConfig = toml::from_str(&config_str)
        .map_err(|e| AppError::ConfigError(format!("Failed to parse config file: {}", e)))?;

    Ok(config)
}

/// Save configuration to file
pub fn save_config(config: &AppConfig) -> AppResult<()> {
    info!("Saving application configuration...");

    let config_dir = dirs::config_dir()
        .ok_or_else(|| AppError::ConfigError("Failed to get config directory".to_string()))?
        .join("code-ai-assistant");

    // Create config directory if it doesn't exist
    std::fs::create_dir_all(&config_dir)
        .map_err(|e| AppError::ConfigError(format!("Failed to create config directory: {}", e)))?;

    let config_file = config_dir.join("config.toml");
    let config_str = toml::to_string_pretty(config)
        .map_err(|e| AppError::ConfigError(format!("Failed to serialize config: {}", e)))?;

    std::fs::write(&config_file, config_str)
        .map_err(|e| AppError::ConfigError(format!("Failed to write config file: {}", e)))?;

    info!("Configuration saved successfully to: {:?}", config_file);
    Ok(())
}