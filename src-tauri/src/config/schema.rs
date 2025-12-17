//! Configuration schema module
//!
//! This module defines additional configuration schemas.

use serde::{Deserialize, Serialize};
use crate::config::loader::{get_default_data_dir, get_user_home};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Application settings
    pub app: AppSettings,
    /// User settings
    pub user: UserSettings,
    /// Database settings
    pub database: DatabaseSettings,
    /// AI service settings
    pub ai: AiSettings,
    /// CLI tool settings
    pub cli: CliToolSettings,
    /// Workspace settings
    pub workspace: WorkspaceSettings,
    //// Deployment settings
    pub deployment: DeploymentSettings,
    /// Logging settings
    pub logging: LoggingSettings,
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
    /// User Home directory
    pub user_home: String,
    /// Enable debug mode
    /// Auto save interval in seconds
    pub auto_save_interval: Option<u32>,
    /// Auto update enabled
    pub auto_update: Option<bool>,
}
/// deployment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSettings {
    /// Deployment environment
    pub environment: String,
    /// DEBUG mode
    pub debug: bool,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
}

/// logging settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    /// Log level
    pub log_level: String,
    /// Log fmt pattern
    pub log_fmt_pattern: Option<String>,
    /// Log file path
    pub log_file_path: String,
    /// Log file name
    pub log_file_name: String,
    /// Enable console logging
    pub console: bool,
    /// file_rotation settings
    pub log_file_rotation: FileRotationSettings,
}

/// File rotation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileRotationSettings {
    /// Maximum file size in MB
    pub log_file_max_size_mb: u64,
    /// Maximum number of backup files
    pub log_file_max_backups: u32,
    /// Maximum age of log files in days
    pub log_file_max_age_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    /// Theme (light/dark)
    pub theme: String,
    /// Font size
    pub font_size: u32,
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
        let data_dir = get_default_data_dir().unwrap();
        Self {
            app: AppSettings {
                name: "Code AI Assistant".to_string(),
                version: "0.1.0".to_string(),
                data_dir: data_dir.clone(),
                user_home: get_user_home().unwrap(),
                auto_save_interval: Some(60),
                auto_update: Some(true),
            },
            user: UserSettings {
                theme: "light".to_string(),
                font_size: 14,
            },
            database: DatabaseSettings {
                url: format!("sqlite://{}/app.db?mode=rwc", data_dir),
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
            deployment: DeploymentSettings {
                environment: "development".to_string(),
                debug: true,
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            logging: LoggingSettings {
                log_level: "debug".to_string(),
                log_file_path: get_default_data_dir().unwrap() + "/logs",
                log_file_name: "app.log".to_string(),
                log_fmt_pattern: Some("%Y-%m-%d %H:%M:%S%.3f %l %T %n %f:%L".to_string()),
                console: true,
                log_file_rotation: FileRotationSettings {
                    log_file_max_size_mb: 10,
                    log_file_max_backups: 5,
                    log_file_max_age_days: 30,
                },
            },
        }
    }
}

/// Environment variable configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    /// Variable name
    pub name: String,
    /// Variable value
    pub value: String,
    /// Is secret (should be masked in UI)
    pub is_secret: bool,
}

/// AI Model configuration for settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Model name
    pub name: String,
    /// API endpoint URL
    pub endpoint: String,
    /// API key (encrypted at rest)
    pub api_key: String,
    /// Is enabled
    pub enabled: bool,
}

/// Code CLI configuration for settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeCliConfig {
    /// CLI name
    pub name: String,
    /// Command path
    pub path: String,
    /// Default arguments
    pub args: String,
    /// Is enabled
    pub enabled: bool,
}

/// Workspace configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    /// Workspace ID
    pub id: String,
    /// Workspace name
    pub name: String,
    /// Workspace root path
    pub path: String,
    /// Created timestamp
    pub created_at: String,
    /// Updated timestamp
    pub updated_at: String,
    /// Associated environment variables
    pub env_vars: Vec<EnvVar>,
    /// Associated models
    pub models: Vec<ModelConfig>,
    /// Associated Code CLIs
    pub code_clis: Vec<CodeCliConfig>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "default".to_string(),
            path: ".".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            env_vars: Vec::new(),
            models: Vec::new(),
            code_clis: Vec::new(),
        }
    }
}

/// Full settings configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsConfig {
    /// Application-wide settings
    pub app: AppWideSettings,
    /// List of workspaces
    pub workspaces: Vec<WorkspaceConfig>,
    /// Active workspace ID
    pub active_workspace: String,
}

/// Application-wide settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppWideSettings {
    /// Theme (light/dark)
    pub theme: String,
    /// Font size
    pub font_size: u32,
    /// Auto-save enabled
    pub auto_save: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: u32,
    /// Data directory path
    pub data_dir: String,
}

impl Default for AppWideSettings {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
            font_size: 14,
            auto_save: true,
            auto_save_interval: 30,
            data_dir: "data".to_string(),
        }
    }
}

impl Default for SettingsConfig {
    fn default() -> Self {
        Self {
            app: AppWideSettings::default(),
            workspaces: vec![WorkspaceConfig::default()],
            active_workspace: "default".to_string(),
        }
    }
}
