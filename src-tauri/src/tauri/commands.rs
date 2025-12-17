//! Tauri commands module
//!
//! This module defines Tauri IPC commands that can be called from the frontend.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle};
use tracing::{error, info};
use tauri::async_runtime;

use crate::config::AppConfig;
use crate::core::AppState;
use super::events::emit_ai_response;

/// File entry for directory listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: u64,
    pub modified: Option<String>,
}

/// Workspace information returned to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceInfo {
    pub id: String,
    pub name: String,
    pub path: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

fn workspaces_file_path(config: &AppConfig) -> PathBuf {
    let mut path = PathBuf::from(&config.app.data_dir);
    path.push("workspaces.json");
    path
}

fn load_workspaces(config: &AppConfig) -> Result<Vec<WorkspaceInfo>, String> {
    let path = workspaces_file_path(config);

    if !path.exists() {
        // 初始化一个默认工作区
        let now = chrono::Utc::now().to_rfc3339();
        let default = WorkspaceInfo {
            id: "default".to_string(),
            name: "default".to_string(),
            path: config.app.data_dir.clone(),
            created_at: now.clone(),
            updated_at: now,
        };

        save_workspaces(config, &[default.clone()])?;
        return Ok(vec![default]);
    }

    let data = fs::read(&path).map_err(|e| e.to_string())?;
    if data.is_empty() {
        return Ok(Vec::new());
    }

    serde_json::from_slice(&data).map_err(|e| e.to_string())
}

fn save_workspaces(config: &AppConfig, workspaces: &[WorkspaceInfo]) -> Result<(), String> {
    let path = workspaces_file_path(config);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }

    let data = serde_json::to_vec_pretty(workspaces).map_err(|e| e.to_string())?;
    fs::write(path, data).map_err(|e| e.to_string())
}

/// Read file content
#[tauri::command]
pub async fn read_file(path: String) -> Result<String, String> {
    info!("Reading file: {}", path);
    // 先检查元数据，避免将目录或超大文件直接读入内存导致应用卡死
    let metadata = fs::metadata(&path).map_err(|e| {
        error!("Failed to stat file {}: {:?}", path, e);
        e.to_string()
    })?;

    if metadata.is_dir() {
        return Err("指定路径是目录，无法作为文件读取".to_string());
    }

    // 简单限制文件大小，避免一次性读取超大文件导致前端/后端卡死
    const MAX_FILE_SIZE: u64 = 8 * 1024 * 1024; // 8MB
    if metadata.len() > MAX_FILE_SIZE {
        let msg = format!(
            "文件过大（{} 字节），当前最大支持 {} 字节，请在外部编辑器中打开",
            metadata.len(), MAX_FILE_SIZE
        );
        error!("{} - path: {}", msg, path);
        return Err(msg);
    }

    // 在阻塞线程池中读取文件，避免阻塞异步运行时
    let read_path = path.clone();
    let bytes = async_runtime::spawn_blocking(move || fs::read(&read_path))
        .await
        .map_err(|e| {
            let msg = format!("Failed to join blocking read task for {}: {:?}", path, e);
            error!("{}", msg);
            msg
        })?
        .map_err(|e| {
            // 额外输出错误日志以便调试
            error!("Failed to read file {}: {:?}", path, e);
            e.to_string()
        })?;

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

    // NOTE: 这里仍然是占位实现，只是演示如何携带关联文件信息
    let snippet_limit: usize = 200;
    let mut file_summaries = Vec::new();

    if let Some(files) = &context_files {
        for path in files.iter().take(8) {
            // 为了避免阻塞，这里只尝试快速读取一小段内容，不影响主线程
            let path_clone = path.clone();
            let result = async_runtime::spawn_blocking(move || fs::read_to_string(&path_clone)).await;

            match result {
                Ok(Ok(content)) => {
                    let preview: String = if content.len() > snippet_limit {
                        format!("{}...", &content[..snippet_limit])
                    } else {
                        content
                    };
                    file_summaries.push(format!("- {}\n{}", path, preview));
                }
                Ok(Err(e)) => {
                    error!("Failed to read context file {}: {:?}", path, e);
                    file_summaries.push(format!("- {} (读取失败: {})", path, e));
                }
                Err(e) => {
                    error!("Failed to join blocking task for context file {}: {:?}", path, e);
                    file_summaries.push(format!("- {} (读取任务失败)", path));
                }
            }
        }
    }

    let base = format!(
        "AI Response: Received your message about '{}'.",
        if message.len() > 50 { &message[..50] } else { &message }
    );

    let response = if file_summaries.is_empty() {
        base
    } else {
        format!(
            "{}\n\nAssociated files (preview):\n{}",
            base,
            file_summaries.join("\n\n")
        )
    };

    Ok(response)
}

/// Send chat message to AI with simulated streaming response
#[tauri::command]
pub async fn send_chat_message_streaming(
    app_handle: AppHandle,
    message: String,
    context_files: Option<Vec<String>>,
) -> Result<String, String> {
    info!("Sending chat message (streaming): {}", message);

    // 为本次会话生成唯一 request_id，前端用它关联流式回复
    let request_id = uuid::Uuid::new_v4().to_string();
    let request_id_for_task = request_id.clone();
    let app_handle_clone = app_handle.clone();

    // 将实际消息处理与流式发送放到后台任务中，避免阻塞当前命令
    let msg = message.clone();
    let ctx_files = context_files.clone();

    async_runtime::spawn(async move {
        // 复用现有的 send_chat_message 逻辑构造完整回复
        match send_chat_message(msg, ctx_files).await {
            Ok(full_response) => {
                let chars: Vec<char> = full_response.chars().collect();
                let total = chars.len();
                let mut buffer = String::new();

                for (idx, ch) in chars.into_iter().enumerate() {
                    buffer.push(ch);

                    let is_last = idx + 1 == total;
                    // 每凑够一定长度，或者到达结尾，就发送一块增量
                    if buffer.len() >= 32 || is_last {
                        let delta = buffer.clone();
                        buffer.clear();

                        if let Err(e) = emit_ai_response(
                            &app_handle_clone,
                            &request_id_for_task,
                            &delta,
                            is_last,
                        ) {
                            error!("Failed to emit AI response chunk: {:?}", e);
                            break;
                        }

                        // 模拟流式延迟效果（阻塞当前后台任务线程即可）
                        std::thread::sleep(Duration::from_millis(60));
                    }
                }
            }
            Err(e) => {
                error!("Failed to build AI response for streaming: {}", e);
                let _ = emit_ai_response(
                    &app_handle_clone,
                    &request_id_for_task,
                    &format!("[AI 错误] {}", e),
                    true,
                );
            }
        }
    });

    // 立即把 request_id 返回给前端，前端可用它在 Chat Messages Area 中关联消息
    Ok(request_id)
}

/// Get available AI models (ordered with current default model first)
#[tauri::command]
pub async fn get_ai_models(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    info!("Getting AI models");

    // 当前支持的模型列表，可以后续改为从配置中动态加载
    let mut models = vec![
        "claude-3-5-sonnet".to_string(),
        "gpt-4".to_string(),
        "gpt-3.5-turbo".to_string(),
        "gemini-pro".to_string(),
    ];

    // 将配置中的默认模型放到列表首位，方便前端直接使用 aiModels[0]
    if let Ok(cfg) = state.config.lock() {
        let current = &cfg.ai.default_model;
        if let Some(pos) = models.iter().position(|m| m == current) {
            if pos != 0 {
                models.swap(0, pos);
            }
        }
    }

    Ok(models)
}

/// Set current AI model and persist to configuration
#[tauri::command]
pub async fn set_ai_model(state: State<'_, AppState>, model: String) -> Result<(), String> {
    info!("Setting AI model to: {}", model);

    {
        // 更新内存中的配置
        let mut cfg = state.config.lock().map_err(|e| e.to_string())?;
        cfg.ai.default_model = model.clone();
        // 同步写入配置文件
        crate::config::save_config(&cfg).map_err(|e| e.to_string())?;
    }

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

/// Execute a command in an existing terminal session
#[tauri::command]
pub async fn execute_terminal_command(
    state: State<'_, AppState>,
    session_id: String,
    command: String,
    args: Vec<String>,
) -> Result<String, String> {
    info!(
        "Executing terminal command in session {}: {} {:?}",
        session_id, command, args
    );

    state
        .terminal
        .execute_command(&session_id, &command, &args)
        .map_err(|e| e.to_string())
}

/// Spawn new terminal session using TerminalService
#[tauri::command]
pub async fn spawn_terminal(state: State<'_, AppState>, cwd: Option<String>) -> Result<String, String> {
    info!("Spawning new terminal");

    state
        .terminal
        .create_session(None, cwd)
        .map_err(|e| e.to_string())
}

/// Kill terminal session via TerminalService
#[tauri::command]
pub async fn kill_terminal(state: State<'_, AppState>, terminal_id: String) -> Result<(), String> {
    info!("Killing terminal: {}", terminal_id);

    state
        .terminal
        .kill_session(&terminal_id)
        .map_err(|e| e.to_string())
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

/// Get workspaces (persisted under data_dir/workspaces.json)
#[tauri::command]
pub async fn get_workspaces(state: State<'_, AppState>) -> Result<Vec<WorkspaceInfo>, String> {
    info!("Getting workspaces");

    let cfg = state.config.lock().map_err(|e| e.to_string())?;
    load_workspaces(&cfg)
}

/// Create workspace and persist to workspaces.json
#[tauri::command]
pub async fn create_workspace(state: State<'_, AppState>, name: String) -> Result<(), String> {
    info!("Creating workspace: {}", name);

    let cfg = state.config.lock().map_err(|e| e.to_string())?;
    let mut list = load_workspaces(&cfg)?;

    if list.iter().any(|w| w.name == name) {
        return Ok(()); // 已存在则忽略
    }

    let now = chrono::Utc::now().to_rfc3339();
    let id = uuid::Uuid::new_v4().to_string();

    let mut path = PathBuf::from(&cfg.app.data_dir);
    path.push("workspaces");
    path.push(&name);

    let path_str = path.to_string_lossy().to_string();
    // 尝试创建目录（失败不致命）
    let _ = fs::create_dir_all(&path);

    let ws = WorkspaceInfo {
        id,
        name,
        path: path_str,
        created_at: now.clone(),
        updated_at: now,
    };

    list.push(ws);
    save_workspaces(&cfg, &list)?;
    Ok(())
}

/// Switch workspace: only update default in config for now
#[tauri::command]
pub async fn switch_workspace(state: State<'_, AppState>, name: String) -> Result<(), String> {
    info!("Switching to workspace: {}", name);

    let mut cfg = state.config.lock().map_err(|e| e.to_string())?;
    let list = load_workspaces(&cfg)?;
    if !list.iter().any(|w| w.name == name) {
        return Err(format!("Workspace not found: {}", name));
    }

    cfg.workspace.default_workspace = name;
    crate::config::save_config(&cfg).map_err(|e| e.to_string())?;
    Ok(())
}

/// Delete workspace from workspaces.json (does not delete files on disk)
#[tauri::command]
pub async fn delete_workspace(state: State<'_, AppState>, name: String) -> Result<(), String> {
    info!("Deleting workspace: {}", name);

    let mut cfg = state.config.lock().map_err(|e| e.to_string())?;
    let mut list = load_workspaces(&cfg)?;

    list.retain(|w| w.name != name);
    save_workspaces(&cfg, &list)?;

    // 如果删除的是默认工作区，则回退到 "default"
    if cfg.workspace.default_workspace == name {
        cfg.workspace.default_workspace = "default".to_string();
        crate::config::save_config(&cfg).map_err(|e| e.to_string())?;
    }

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

/// Get application logs from the configured log file
#[tauri::command]
pub async fn get_logs(state: State<'_, AppState>, limit: Option<usize>) -> Result<Vec<String>, String> {
    info!("Getting application logs");

    let cfg = state.config.lock().map_err(|e| e.to_string())?;
    let mut path = PathBuf::from(&cfg.logging.log_file_path);
    path.push(&cfg.logging.log_file_name);

    if !path.exists() {
        return Ok(Vec::new());
    }

    use std::io::{BufRead, BufReader};

    let file = fs::File::open(&path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .collect();

    if let Some(limit) = limit {
        if lines.len() > limit {
            lines = lines.split_off(lines.len() - limit);
        }
    }

    Ok(lines)
}

/// Clear application logs by truncating the log file
#[tauri::command]
pub async fn clear_logs(state: State<'_, AppState>) -> Result<(), String> {
    info!("Clearing application logs");

    let cfg = state.config.lock().map_err(|e| e.to_string())?;
    let mut path = PathBuf::from(&cfg.logging.log_file_path);
    path.push(&cfg.logging.log_file_name);

    if path.exists() {
        fs::write(&path, "").map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Get a single setting by key
#[tauri::command]
pub async fn get_setting(app: AppHandle, key: String) -> Result<Option<serde_json::Value>, String> {
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
    app: AppHandle,
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
    app: AppHandle,
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
    app: AppHandle,
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
    app: AppHandle,
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
    app: AppHandle,
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
