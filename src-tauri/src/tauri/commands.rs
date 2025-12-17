//! Tauri commands module
//!
//! This module defines Tauri IPC commands that can be called from the frontend.

use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

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
    // 先按字节读取，再使用 UTF-8 有损解码，避免非 UTF-8 文件导致报错
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    let content = String::from_utf8_lossy(&bytes).to_string();
    Ok(content)
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
pub async fn get_settings(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    info!("Getting application settings");
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    let settings = crate::database::repositories::settings_repository::SettingsRepository::get_all(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    // Convert settings to JSON object
    let mut settings_map = serde_json::Map::new();
    for setting in settings {
        let value: serde_json::Value = serde_json::from_str(&setting.value)
            .unwrap_or(serde_json::Value::String(setting.value.clone()));
        settings_map.insert(setting.key, value);
    }
    
    Ok(serde_json::Value::Object(settings_map))
}

/// Save application settings
#[tauri::command]
pub async fn save_settings(
    app: tauri::AppHandle,
    settings: serde_json::Value,
) -> Result<(), String> {
    info!("Saving application settings");

    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    // If settings is an object, save each key-value pair
    if let serde_json::Value::Object(map) = settings {
        for (key, value) in map {
            let value_str = serde_json::to_string(&value).map_err(|e| e.to_string())?;
            
            // Determine category from key prefix
            let category = if key.starts_with("app.") {
                "app"
            } else if key.starts_with("user.") {
                "user"
            } else if key.starts_with("workspace.") {
                "workspace"
            } else if key.starts_with("ai.") {
                "ai"
            } else {
                "general"
            };
            
            crate::database::repositories::settings_repository::SettingsRepository::upsert(
                &db,
                &key,
                &value_str,
                category,
                None,
            )
            .await
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

/// Reset settings to defaults
#[tauri::command]
pub async fn reset_settings(app: tauri::AppHandle) -> Result<(), String> {
    info!("Resetting settings to defaults");
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    // Delete all user-modifiable settings
    crate::database::repositories::settings_repository::SettingsRepository::delete_by_category(&db, "user")
        .await
        .map_err(|e| e.to_string())?;
    crate::database::repositories::settings_repository::SettingsRepository::delete_by_category(&db, "workspace")
        .await
        .map_err(|e| e.to_string())?;
    crate::database::repositories::settings_repository::SettingsRepository::delete_by_category(&db, "general")
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
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

/// Get a single setting by key
#[tauri::command]
pub async fn get_setting(app: tauri::AppHandle, key: String) -> Result<Option<serde_json::Value>, String> {
    info!("Getting setting: {}", key);
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    let setting = crate::database::repositories::settings_repository::SettingsRepository::get_by_key(&db, &key)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(setting.map(|s| {
        serde_json::from_str(&s.value)
            .unwrap_or(serde_json::Value::String(s.value))
    }))
}

/// Save a single setting
#[tauri::command]
pub async fn save_setting(
    app: tauri::AppHandle,
    key: String,
    value: serde_json::Value,
    category: Option<String>,
) -> Result<(), String> {
    info!("Saving setting: {}", key);
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    let value_str = serde_json::to_string(&value).map_err(|e| e.to_string())?;
    
    // Determine category from key prefix if not provided
    let cat = category.unwrap_or_else(|| {
        if key.starts_with("app.") {
            "app"
        } else if key.starts_with("user.") {
            "user"
        } else if key.starts_with("workspace.") {
            "workspace"
        } else if key.starts_with("ai.") {
            "ai"
        } else {
            "general"
        }.to_string()
    });
    
    crate::database::repositories::settings_repository::SettingsRepository::upsert(
        &db,
        &key,
        &value_str,
        &cat,
        None,
    )
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Get settings by category
#[tauri::command]
pub async fn get_settings_by_category(
    app: tauri::AppHandle,
    category: String,
) -> Result<serde_json::Value, String> {
    info!("Getting settings for category: {}", category);
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    let settings = crate::database::repositories::settings_repository::SettingsRepository::get_by_category(&db, &category)
        .await
        .map_err(|e| e.to_string())?;
    
    let mut settings_map = serde_json::Map::new();
    for setting in settings {
        let value: serde_json::Value = serde_json::from_str(&setting.value)
            .unwrap_or(serde_json::Value::String(setting.value.clone()));
        settings_map.insert(setting.key, value);
    }
    
    Ok(serde_json::Value::Object(settings_map))
}

/// Add a recent directory
#[tauri::command]
pub async fn add_recent_directory(
    app: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    info!("Adding recent directory: {}", path);
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    crate::database::repositories::recent_directories_repository::RecentDirectoriesRepository::add(&db, &path)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

/// Get recent directories
#[tauri::command]
pub async fn get_recent_directories(
    app: tauri::AppHandle,
) -> Result<Vec<serde_json::Value>, String> {
    info!("Getting recent directories");
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    let directories = crate::database::repositories::recent_directories_repository::RecentDirectoriesRepository::get_recent(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    let result = directories.into_iter().map(|dir| {
        serde_json::json!({
            "path": dir.path,
            "openedAt": dir.opened_at.to_rfc3339(),
        })
    }).collect();
    
    Ok(result)
}

/// Clear recent directories
#[tauri::command]
pub async fn clear_recent_directories(
    app: tauri::AppHandle,
) -> Result<(), String> {
    info!("Clearing recent directories");
    
    let db = crate::database::connection::get_db_connection(&app)
        .await
        .map_err(|e| e.to_string())?;
    
    crate::database::repositories::recent_directories_repository::RecentDirectoriesRepository::clear(&db)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
}
