//! Tauri commands module
//!
//! This module defines Tauri IPC commands that can be called from the frontend.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tauri::State;
use tracing::{error, info};

use crate::config::AppConfig;
use crate::core::AppState;

/// File entry for directory listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: Option<String>,
}

/// Read file content
#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    info!("Reading file: {}", path);
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

/// Write file content
#[tauri::command]
pub async fn write_file(path: String, content: String) -> Result<(), String> {
    info!("Writing file: {}", path);
    fs::write(&path, content).map_err(|e| e.to_string())
}

/// List files in directory
#[tauri::command]
pub async fn list_files(path: String) -> Result<Vec<FileEntry>, String> {
    info!("Listing files in: {}", path);

    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    let mut files = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let metadata = entry.metadata().map_err(|e| e.to_string())?;
        let path_buf = entry.path();

        files.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: path_buf.to_string_lossy().to_string(),
            is_directory: metadata.is_dir(),
            size: metadata.len(),
            modified: metadata.modified().ok().map(|t| {
                let datetime: chrono::DateTime<chrono::Utc> = t.into();
                datetime.to_rfc3339()
            }),
        });
    }

    // Sort: directories first, then by name
    files.sort_by(|a, b| {
        match (a.is_directory, b.is_directory) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(files)
}

/// Create file
#[tauri::command]
pub async fn create_file(path: String) -> Result<(), String> {
    info!("Creating file: {}", path);
    fs::File::create(&path).map_err(|e| e.to_string())?;
    Ok(())
}

/// Delete file
#[tauri::command]
pub async fn delete_file(path: String) -> Result<(), String> {
    info!("Deleting file: {}", path);
    let path = Path::new(&path);
    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| e.to_string())
    } else {
        fs::remove_file(path).map_err(|e| e.to_string())
    }
}

/// Rename file
#[tauri::command]
pub async fn rename_file(old_path: String, new_path: String) -> Result<(), String> {
    info!("Renaming file: {} -> {}", old_path, new_path);
    fs::rename(&old_path, &new_path).map_err(|e| e.to_string())
}

/// Create directory
#[tauri::command]
pub async fn create_directory(path: String) -> Result<(), String> {
    info!("Creating directory: {}", path);
    fs::create_dir_all(&path).map_err(|e| e.to_string())
}

/// List directories
#[tauri::command]
pub async fn list_directories(path: String) -> Result<Vec<String>, String> {
    info!("Listing directories in: {}", path);

    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            dirs.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    Ok(dirs)
}

/// Delete directory
#[tauri::command]
pub async fn delete_directory(path: String) -> Result<(), String> {
    info!("Deleting directory: {}", path);
    fs::remove_dir_all(&path).map_err(|e| e.to_string())
}

/// Send chat message to AI
#[tauri::command]
pub async fn send_chat_message(
    message: String,
    context_files: Option<Vec<String>>,
) -> Result<String, String> {
    info!("Sending chat message: {}", message);

    // TODO: Implement actual AI service integration
    let response = format!(
        "AI Response: Received your message about '{}'. Context files: {:?}",
        if message.len() > 50 { &message[..50] } else { &message },
        context_files.as_ref().map(|f| f.len()).unwrap_or(0)
    );

    Ok(response)
}

/// Get available AI models
#[tauri::command]
pub async fn get_ai_models() -> Result<Vec<String>, String> {
    info!("Getting AI models");

    // TODO: Load from configuration
    let models = vec![
        "claude-3-5-sonnet".to_string(),
        "gpt-4".to_string(),
        "gpt-3.5-turbo".to_string(),
        "gemini-pro".to_string(),
    ];

    Ok(models)
}

/// Set current AI model
#[tauri::command]
pub async fn set_ai_model(model: String) -> Result<(), String> {
    info!("Setting AI model to: {}", model);
    // TODO: Save to configuration
    Ok(())
}

/// Execute command in terminal
#[tauri::command]
pub async fn execute_command(
    command: String,
    args: Vec<String>,
    cwd: Option<String>,
) -> Result<String, String> {
    info!("Executing command: {} {:?}", command, args);

    let mut cmd = std::process::Command::new(&command);
    cmd.args(&args);

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    let output = cmd.output().map_err(|e| e.to_string())?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if !stderr.is_empty() {
        error!("Command stderr: {}", stderr);
    }

    Ok(stdout)
}

/// Spawn new terminal
#[tauri::command]
pub async fn spawn_terminal() -> Result<String, String> {
    info!("Spawning new terminal");

    // Return a placeholder terminal ID
    let terminal_id = uuid::Uuid::new_v4().to_string();
    Ok(terminal_id)
}

/// Kill terminal
#[tauri::command]
pub async fn kill_terminal(terminal_id: String) -> Result<(), String> {
    info!("Killing terminal: {}", terminal_id);
    // TODO: Implement terminal killing
    Ok(())
}

/// Get application settings
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppConfig, String> {
    info!("Getting application settings");
    let config = state.config.lock().map_err(|e| e.to_string())?;
    Ok(config.clone())
}

/// Save application settings
#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    info!("Saving application settings");

    crate::config::save_config(&config).map_err(|e| e.to_string())?;

    // Update state
    let mut state_config = state.config.lock().map_err(|e| e.to_string())?;
    *state_config = config;

    Ok(())
}

/// Reset settings to defaults
#[tauri::command]
pub async fn reset_settings(state: State<'_, AppState>) -> Result<AppConfig, String> {
    info!("Resetting settings to defaults");

    let default_config = AppConfig::default();
    crate::config::save_config(&default_config).map_err(|e| e.to_string())?;

    // Update state
    let mut state_config = state.config.lock().map_err(|e| e.to_string())?;
    *state_config = default_config.clone();

    Ok(default_config)
}

/// Get workspaces
#[tauri::command]
pub async fn get_workspaces() -> Result<Vec<String>, String> {
    info!("Getting workspaces");

    // TODO: Load from database
    let workspaces = vec![
        "default".to_string(),
        "project-1".to_string(),
        "project-2".to_string(),
    ];

    Ok(workspaces)
}

/// Create workspace
#[tauri::command]
pub async fn create_workspace(name: String) -> Result<(), String> {
    info!("Creating workspace: {}", name);
    // TODO: Save to database
    Ok(())
}

/// Switch workspace
#[tauri::command]
pub async fn switch_workspace(name: String) -> Result<(), String> {
    info!("Switching to workspace: {}", name);
    // TODO: Load from database
    Ok(())
}

/// Delete workspace
#[tauri::command]
pub async fn delete_workspace(name: String) -> Result<(), String> {
    info!("Deleting workspace: {}", name);
    // TODO: Delete from database
    Ok(())
}

/// Get system information
#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    info!("Getting system information");

    use sysinfo::System;

    let mut sys = System::new_all();
    sys.refresh_all();

    let info = serde_json::json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "total_memory": sys.total_memory(),
        "used_memory": sys.used_memory(),
        "total_swap": sys.total_swap(),
        "used_swap": sys.used_swap(),
        "cpu_count": sys.cpus().len(),
        "host_name": System::host_name().unwrap_or_default(),
    });

    Ok(info)
}

/// Get application logs
#[tauri::command]
pub async fn get_logs(limit: Option<usize>) -> Result<Vec<String>, String> {
    info!("Getting application logs");

    // TODO: Read from log file
    let logs = vec![
        format!("{} INFO - Application started", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")),
        format!("{} DEBUG - Loading configuration", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")),
        format!("{} INFO - Database connected", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")),
    ];

    let result = if let Some(limit) = limit {
        logs.into_iter().take(limit).collect()
    } else {
        logs
    };

    Ok(result)
}

/// Clear application logs
#[tauri::command]
pub async fn clear_logs() -> Result<(), String> {
    info!("Clearing application logs");
    // TODO: Clear log file
    Ok(())
}
