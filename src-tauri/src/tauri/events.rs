//! Tauri events module
//!
//! This module defines Tauri event handlers.

use tauri::{AppHandle, Emitter, Listener};
use tracing::info;

use crate::utils::error::{AppError, AppResult};

/// Register event handlers
pub fn register_event_handlers(app: &mut tauri::App) -> AppResult<()> {
    info!("Registering event handlers...");

    // Set up event listeners
    let app_handle = app.handle();
    
    app_handle.listen("file-changed", |event| {
        info!("File changed event: {:?}", event.payload());
    });

    app_handle.listen("terminal-output", |event| {
        info!("Terminal output event: {:?}", event.payload());
    });

    app_handle.listen("ai-response", |event| {
        info!("AI response event: {:?}", event.payload());
    });

    app_handle.listen("log-message", |event| {
        info!("Log message event: {:?}", event.payload());
    });

    info!("Event handlers registered successfully");
    Ok(())
}

/// Emit file changed event
pub fn emit_file_changed(app_handle: &AppHandle, path: &str, operation: &str) -> AppResult<()> {
    let payload = serde_json::json!({
        "path": path,
        "operation": operation,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("file-changed", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}

/// Emit terminal output event
pub fn emit_terminal_output(app_handle: &AppHandle, terminal_id: &str, output: &str) -> AppResult<()> {
    let payload = serde_json::json!({
        "terminal_id": terminal_id,
        "output": output,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("terminal-output", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}

/// Emit AI response event
pub fn emit_ai_response(app_handle: &AppHandle, request_id: &str, response: &str) -> AppResult<()> {
    let payload = serde_json::json!({
        "request_id": request_id,
        "response": response,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("ai-response", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}

/// Emit log message event
pub fn emit_log_message(app_handle: &AppHandle, level: &str, message: &str) -> AppResult<()> {
    let payload = serde_json::json!({
        "level": level,
        "message": message,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("log-message", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}

/// Emit workspace changed event
pub fn emit_workspace_changed(app_handle: &AppHandle, workspace: &str) -> AppResult<()> {
    let payload = serde_json::json!({
        "workspace": workspace,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("workspace-changed", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}

/// Emit settings updated event
pub fn emit_settings_updated(app_handle: &AppHandle) -> AppResult<()> {
    let payload = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("settings-updated", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}

/// Emit application ready event
pub fn emit_app_ready(app_handle: &AppHandle) -> AppResult<()> {
    let payload = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("app-ready", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}

/// Emit error event
pub fn emit_error(app_handle: &AppHandle, error: &str, details: Option<&str>) -> AppResult<()> {
    let payload = serde_json::json!({
        "error": error,
        "details": details,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    app_handle.emit("error", payload.to_string())
        .map_err(|e| AppError::TauriError(e))
}