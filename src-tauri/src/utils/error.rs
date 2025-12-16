//! Error handling module
//!
//! This module defines application-wide error types and utilities.

use thiserror::Error;

/// Application result type
pub type AppResult<T> = Result<T, AppError>;

/// Application error type
#[derive(Error, Debug)]
pub enum AppError {
    /// IO errors
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Database errors
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Network errors
    #[error("Network error: {0}")]
    NetworkError(String),

    /// Serialization/deserialization errors
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Validation errors
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// AI service errors
    #[error("AI service error: {0}")]
    AiServiceError(String),

    /// File system errors
    #[error("File system error: {0}")]
    FileSystemError(String),

    /// Process execution errors
    #[error("Process execution error: {0}")]
    ProcessError(String),

    /// Tauri errors
    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),

    /// Anyhow errors
    #[error("Other error: {0}")]
    AnyhowError(#[from] anyhow::Error),

    /// Generic errors
    #[error("Error: {0}")]
    GenericError(String),
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/// Convert SeaORM error to AppError
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

/// Convert reqwest error to AppError
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::NetworkError(err.to_string())
    }
}

/// Convert serde_json error to AppError
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

/// Convert config error to AppError
impl From<config::ConfigError> for AppError {
    fn from(err: config::ConfigError) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

/// Helper function to create validation errors
pub fn validation_error(message: &str) -> AppError {
    AppError::ValidationError(message.to_string())
}

/// Helper function to create generic errors
pub fn generic_error(message: &str) -> AppError {
    AppError::GenericError(message.to_string())
}