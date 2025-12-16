//! Configuration schema module
//!
//! This module defines additional configuration schemas.

use serde::{Deserialize, Serialize};

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
