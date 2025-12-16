//! AI Service module
//!
//! This module handles communication with AI models and CLI tools.

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::utils::error::{AppError, AppResult};

/// AI Model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModel {
    /// Model name
    pub name: String,
    /// API endpoint
    pub endpoint: String,
    /// API key (encrypted)
    pub api_key: String,
    /// Is active
    pub is_active: bool,
}

/// Code CLI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeCli {
    /// CLI name
    pub name: String,
    /// Command path
    pub command_path: String,
    /// Command arguments
    pub arguments: Vec<String>,
    /// Is active
    pub is_active: bool,
}

/// AI Service for managing AI models and sending messages
pub struct AiService {
    /// Available AI models
    models: Vec<AiModel>,
    /// Available Code CLIs
    code_clis: Vec<CodeCli>,
    /// Current model
    current_model: Option<String>,
    /// HTTP client
    _client: reqwest::Client,
}

impl AiService {
    /// Create a new AI service
    pub fn new() -> Self {
        Self {
            models: vec![
                AiModel {
                    name: "claude-3-5-sonnet".to_string(),
                    endpoint: "https://api.anthropic.com/v1/messages".to_string(),
                    api_key: String::new(),
                    is_active: true,
                },
                AiModel {
                    name: "gpt-4".to_string(),
                    endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
                    api_key: String::new(),
                    is_active: true,
                },
                AiModel {
                    name: "gpt-3.5-turbo".to_string(),
                    endpoint: "https://api.openai.com/v1/chat/completions".to_string(),
                    api_key: String::new(),
                    is_active: true,
                },
                AiModel {
                    name: "gemini-pro".to_string(),
                    endpoint: "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent".to_string(),
                    api_key: String::new(),
                    is_active: true,
                },
            ],
            code_clis: Vec::new(),
            current_model: Some("claude-3-5-sonnet".to_string()),
            _client: reqwest::Client::new(),
        }
    }

    /// Get available models
    pub fn get_models(&self) -> Vec<String> {
        self.models
            .iter()
            .filter(|m| m.is_active)
            .map(|m| m.name.clone())
            .collect()
    }

    /// Set current model
    pub fn set_current_model(&mut self, model: String) -> AppResult<()> {
        if self.models.iter().any(|m| m.name == model) {
            self.current_model = Some(model);
            Ok(())
        } else {
            Err(AppError::ConfigError(format!("Model not found: {}", model)))
        }
    }

    /// Send message to AI (placeholder)
    pub async fn send_message(
        &self,
        message: &str,
        _context_files: Option<Vec<String>>,
    ) -> AppResult<String> {
        info!("Sending message to AI: {}", message);

        // TODO: Implement actual AI API calls
        // For now, return a placeholder response
        let response = format!(
            "AI Response (Model: {}): Received your message about '{}'",
            self.current_model.as_deref().unwrap_or("unknown"),
            if message.len() > 50 {
                &message[..50]
            } else {
                message
            }
        );

        Ok(response)
    }

    /// Add a new model
    pub fn add_model(&mut self, model: AiModel) {
        self.models.push(model);
    }

    /// Remove a model
    pub fn remove_model(&mut self, name: &str) {
        self.models.retain(|m| m.name != name);
    }

    /// Add a new Code CLI
    pub fn add_code_cli(&mut self, cli: CodeCli) {
        self.code_clis.push(cli);
    }

    /// Remove a Code CLI
    pub fn remove_code_cli(&mut self, name: &str) {
        self.code_clis.retain(|c| c.name != name);
    }

    /// Get available Code CLIs
    pub fn get_code_clis(&self) -> Vec<String> {
        self.code_clis
            .iter()
            .filter(|c| c.is_active)
            .map(|c| c.name.clone())
            .collect()
    }
}

impl Default for AiService {
    fn default() -> Self {
        Self::new()
    }
}
