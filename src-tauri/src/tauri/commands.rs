//! Tauri commands module
//!
//! This module defines Tauri IPC commands that can be called from the frontend.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::{Arc, Mutex};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use tauri::{AppHandle, Manager, State};
use tracing::{error, info, debug, warn};
use tauri::async_runtime;
use tokio::io::{AsyncRead, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;
use tokio::sync::oneshot;
use tokio::time::sleep;
use serde_json::Value;
use crate::core::{AppState, app::StreamingTaskHandle};
use crate::services::ai::{AiChatOptions, AiService};
use crate::services::chat_session::{self, ChatMessage};
use crate::utils::error::AppError;
use super::event_handlers::emit_ai_response;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// Send chat message to AI
#[tauri::command]
pub async fn send_chat_message(
    message: String,
    context_files: Option<Vec<String>>,
) -> Result<String, String> {
    debug!("Sending chat message: {}", message);

    // Use AiService as the single entry; internally it calls codeagent-wrapper.
    let ai = AiService::new();
    ai.send_message(&message, context_files)
        .await
        .map_err(|e| e.to_string())
}

/// Send chat message to AI with simulated streaming response
#[tauri::command]
pub async fn send_chat_message_streaming(
    app_handle: AppHandle,
    message: String,
    context_files: Option<Vec<String>>,
    code_cli: Option<String>,
    codex_model: Option<String>,
    session_id: Option<String>,
    workspace_id: Option<String>,
    workspace_dir: Option<String>,
    code_cli_changed: Option<bool>,
    code_cli_task_id: Option<String>,
    direct_cli: Option<bool>,
    cli_command: Option<String>,
    cli_args: Option<Vec<String>>,
) -> Result<String, String> {
    debug!("Sending chat message (streaming): {}", message);
    debug!(
        code_cli = ?code_cli,
        session_id = ?session_id,
        codex_model = ?codex_model,
        code_cli_changed = ?code_cli_changed,
        code_cli_task_id = ?code_cli_task_id,
        "Streaming chat options"
    );
    // let db = crate::database::connection::get_db_connection(&app_handle)
    //     .await
    //     .map_err(|e| e.to_string())?;
    
    // let workspace_id_parsed = workspace_id
    //     .as_ref()
    //     .and_then(|id| id.parse::<i32>().ok())
    //     .ok_or_else(|| "Invalid workspace_id".to_string())?;
    
    // let workspace = crate::database::repositories::workspace_repository::WorkspaceRepository::get_by_id(&db, &workspace_id_parsed)
    //     .await
    //     .map_err(|e| e.to_string())?
    //     .ok_or_else(|| "Workspace not found".to_string())?;

    // 生成session_id
    let session_id = session_id.unwrap_or(uuid::Uuid::new_v4().to_string());
    debug!("Session ID: {}", session_id);
    let config = crate::core::app::get_config(app_handle.state::<AppState>());
    // let app_handle_clone = app_handle.clone();
    let request_id = uuid::Uuid::new_v4().to_string();
    let request_id_for_task = request_id.clone();
    let (cancel_tx, cancel_rx) = oneshot::channel::<()>();

    // Spawn the streaming task in the background.
    let msg = message.clone();
    let ctx_files = context_files.clone();
    let msg_for_spawn = msg.clone();

    let workspace_id_for_append = workspace_id.clone();
    let code_cli_for_task = code_cli.clone();
    let code_cli_for_append = code_cli.clone();
    let codex_model_for_task = codex_model.clone();
    let workspace_dir_for_task = workspace_dir.clone();
    let code_cli_task_id_for_resume = code_cli_task_id.clone();
    let code_cli_changed_flag = code_cli_changed;
    let direct_cli_enabled = direct_cli.unwrap_or(false);
    let cli_command_for_task = cli_command.clone().unwrap_or_default();
    let cli_args_for_task = cli_args.clone().unwrap_or_default();
    let env_vars_for_task = config.env_vars.clone();

    let app_handle_for_task = app_handle.clone();
    let request_id_for_spawn = request_id_for_task.clone();
    let join_handle = if direct_cli_enabled {
        let cancel_rx = cancel_rx;
        async_runtime::spawn(async move {
            let mut cancel_rx = Some(cancel_rx);
            sleep(Duration::from_millis(30)).await;

            if cli_command_for_task.trim().is_empty() {
                let _ = emit_ai_response(
                    &app_handle_for_task,
                    &request_id_for_spawn,
                    "[AI error] Direct CLI enabled but command is empty.",
                    true,
                    Some(&session_id),
                    workspace_id_for_append.as_deref(),
                    None,
                );
                return;
            }

            let task = AiService::build_task_with_context(&msg, ctx_files.as_deref());
            let workdir = workspace_dir_for_task.clone().unwrap_or_else(|| ".".to_string());
            let backend = code_cli_for_task
                .as_deref()
                .and_then(AiService::derive_backend_from_code_cli)
                .or_else(|| derive_backend_from_command(&cli_command_for_task));
            let direct_plan = build_direct_cli_plan(
                backend.as_deref(),
                &cli_args_for_task,
                code_cli_task_id_for_resume.as_deref(),
                code_cli_changed_flag,
            );
            let direct_args = direct_plan.args;
            let mut direct_task_id = direct_plan.task_id.clone();

            let mut cmd = Command::new(&cli_command_for_task);
            #[cfg(windows)]
            {
                cmd.creation_flags(CREATE_NO_WINDOW);
            }
            cmd.args(&direct_args)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .current_dir(&workdir);
            for (key, value) in &env_vars_for_task {
                cmd.env(key, value);
            }

            let mut child = match cmd.spawn() {
                Ok(child) => child,
                Err(e) => {
                    let _ = emit_ai_response(
                        &app_handle_for_task,
                        &request_id_for_spawn,
                        &format!("[AI error] Failed to start CLI: {}", e),
                        true,
                        Some(&session_id),
                        workspace_id_for_append.as_deref(),
                        None,
                    );
                    return;
                }
            };

            if let Some(mut stdin) = child.stdin.take() {
                let mut input = task.clone();
                if !input.ends_with('\n') {
                    input.push('\n');
                }
                if let Err(e) = stdin.write_all(input.as_bytes()).await {
                    warn!("Failed to write CLI stdin: {}", e);
                }
            }

            let mut stdout_reader = child.stdout.take().map(BufReader::new);
            let mut stderr_reader = child.stderr.take().map(BufReader::new);
            let mut stdout_done = stdout_reader.is_none();
            let mut stderr_done = stderr_reader.is_none();
            let mut stdout_line = String::new();
            let mut stderr_line = String::new();
            let mut full_response = String::new();

            while !stdout_done || !stderr_done {
                if let Some(cancel_fut) = cancel_rx.as_mut() {
                    tokio::select! {
                        _ = cancel_fut => {
                            if let Err(e) = child.kill().await {
                                warn!("Failed to kill direct CLI after cancellation: {}", e);
                            }
                            return;
                        }
                        read = read_line_if_available(&mut stdout_reader, &mut stdout_line), if !stdout_done => {
                            match read {
                                Ok(0) => stdout_done = true,
                                Ok(_) => {
                                    if let Some(id) = parse_cli_session_id(&stdout_line, backend.as_deref()) {
                                        if should_replace_task_id(direct_task_id.as_deref(), &id) {
                                            direct_task_id = Some(id);
                                        }
                                    }
                                    let delta = stdout_line.clone();
                                    full_response.push_str(&delta);
                                    let _ = emit_ai_response(
                                        &app_handle_for_task,
                                        &request_id_for_spawn,
                                        &delta,
                                        false,
                                        Some(&session_id),
                                        workspace_id_for_append.as_deref(),
                                        None,
                                    );
                                }
                                Err(e) => {
                                    warn!("Failed to read CLI stdout: {}", e);
                                    stdout_done = true;
                                }
                            }
                        }
                        read = read_line_if_available(&mut stderr_reader, &mut stderr_line), if !stderr_done => {
                            match read {
                                Ok(0) => stderr_done = true,
                                Ok(_) => {
                                    let delta = format!("[stderr] {}", stderr_line);
                                    full_response.push_str(&delta);
                                    let _ = emit_ai_response(
                                        &app_handle_for_task,
                                        &request_id_for_spawn,
                                        &delta,
                                        false,
                                        Some(&session_id),
                                        workspace_id_for_append.as_deref(),
                                        None,
                                    );
                                }
                                Err(e) => {
                                    warn!("Failed to read CLI stderr: {}", e);
                                    stderr_done = true;
                                }
                            }
                        }
                    }
                } else {
                    tokio::select! {
                        read = read_line_if_available(&mut stdout_reader, &mut stdout_line), if !stdout_done => {
                            match read {
                                Ok(0) => stdout_done = true,
                                Ok(_) => {
                                    if let Some(id) = parse_cli_session_id(&stdout_line, backend.as_deref()) {
                                        if should_replace_task_id(direct_task_id.as_deref(), &id) {
                                            direct_task_id = Some(id);
                                        }
                                    }
                                    let delta = stdout_line.clone();
                                    full_response.push_str(&delta);
                                    let _ = emit_ai_response(
                                        &app_handle_for_task,
                                        &request_id_for_spawn,
                                        &delta,
                                        false,
                                        Some(&session_id),
                                        workspace_id_for_append.as_deref(),
                                        None,
                                    );
                                }
                                Err(e) => {
                                    warn!("Failed to read CLI stdout: {}", e);
                                    stdout_done = true;
                                }
                            }
                        }
                        read = read_line_if_available(&mut stderr_reader, &mut stderr_line), if !stderr_done => {
                            match read {
                                Ok(0) => stderr_done = true,
                                Ok(_) => {
                                    let delta = format!("[stderr] {}", stderr_line);
                                    full_response.push_str(&delta);
                                    let _ = emit_ai_response(
                                        &app_handle_for_task,
                                        &request_id_for_spawn,
                                        &delta,
                                        false,
                                        Some(&session_id),
                                        workspace_id_for_append.as_deref(),
                                        None,
                                    );
                                }
                                Err(e) => {
                                    warn!("Failed to read CLI stderr: {}", e);
                                    stderr_done = true;
                                }
                            }
                        }
                    }
                }
            }

            let exit_status = match child.wait().await {
                Ok(status) => status,
                Err(e) => {
                    let _ = emit_ai_response(
                        &app_handle_for_task,
                        &request_id_for_spawn,
                        &format!("[AI error] Failed to wait for CLI: {}", e),
                        true,
                        Some(&session_id),
                        workspace_id_for_append.as_deref(),
                        None,
                    );
                    return;
                }
            };
            let exit_code = exit_status.code().unwrap_or(-1);
            let success = exit_code == 0;
            if !success {
                let delta = format!("[exit {}] CLI exited with errors\n", exit_code);
                full_response.push_str(&delta);
                let _ = emit_ai_response(
                    &app_handle_for_task,
                    &request_id_for_spawn,
                    &delta,
                    true,
                    Some(&session_id),
                    workspace_id_for_append.as_deref(),
                    None,
                );
            } else {
                let _ = emit_ai_response(
                    &app_handle_for_task,
                    &request_id_for_spawn,
                    "",
                    true,
                    Some(&session_id),
                    workspace_id_for_append.as_deref(),
                    direct_task_id.as_deref(),
                );
            }

            if success {
                let user_message = ChatMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    role: "user".to_string(),
                    content: msg_for_spawn.clone(),
                    timestamp: chrono::Local::now().to_rfc3339().to_string(),
                    files: None,
                    session_id: Some(session_id.clone()),
                    workspace_id: workspace_id_for_append.clone(),
                    model: None,
                };
                let assistant_message = ChatMessage {
                    id: request_id_for_spawn.clone(),
                    role: "assistant".to_string(),
                    content: full_response,
                    timestamp: chrono::Local::now().to_rfc3339(),
                    files: None,
                    session_id: Some(session_id.clone()),
                    workspace_id: workspace_id_for_append.clone(),
                    model: None,
                };
                if let Err(e) = chat_session::append_message_to_session(
                    &session_id,
                    vec![user_message, assistant_message],
                    code_cli_for_append.clone(),
                    direct_task_id.clone(),
                ) {
                    error!(
                        "Failed to append chat messages to session {}: {}",
                        session_id, e
                    );
                }
            }
        })
    } else {
        let cancel_rx = cancel_rx;
        async_runtime::spawn(async move {
            let ai = AiService::new();
            match ai
                .send_message_with_options(
                    &msg,
                    ctx_files,
                    AiChatOptions {
                        code_cli: code_cli_for_task,
                        resume_session_id: code_cli_task_id_for_resume,
                        parallel: false,
                        codex_model: codex_model_for_task,
                        workspace_dir: workspace_dir_for_task,
                        code_cli_changed: code_cli_changed_flag,
                        env: env_vars_for_task,
                        cancel_rx: Some(cancel_rx),
                    },
                )
                .await
            {   
                Ok(result) => {
                    debug!("AI response: {}", result.message);
                    let chars: Vec<char> = result.message.chars().collect();
                    let total = chars.len();
                    let mut buffer = String::new();
                    let mut full_response = String::new();

                    for (idx, ch) in chars.into_iter().enumerate() {
                        buffer.push(ch);

                        let is_last = idx + 1 == total;
                        // Send a chunk once the buffer is big enough or we're at the end.
                        if buffer.len() >= 32 || is_last {
                            let delta = buffer.clone();
                            buffer.clear();

                            let codeagent_session_id = if is_last {
                                result.codeagent_session_id.clone()
                            } else {
                                None
                            };
                            full_response.push_str(&delta);

                            if let Err(e) = emit_ai_response(
                                &app_handle_for_task,
                                &request_id_for_spawn,
                                &delta,
                                is_last,
                                Some(&session_id),
                                workspace_id_for_append.as_deref(),
                                codeagent_session_id.as_deref(),
                            ) {
                                error!("Failed to emit AI response chunk: {:?}", e);
                                break;
                            }

                            // Simulate streaming delay by sleeping inside the task.
                            std::thread::sleep(Duration::from_millis(60));
                        }
                    }
                    
                    if let Some(task_id) = result.codeagent_session_id.clone() {
                        let user_message = ChatMessage {
                            id: uuid::Uuid::new_v4().to_string(),
                            role: "user".to_string(),
                            content: msg_for_spawn.clone(),
                            timestamp: chrono::Local::now().to_rfc3339().to_string(),
                            files: None,
                            session_id: Some(session_id.clone()),
                            workspace_id: workspace_id_for_append.clone(),
                            model: None,
                        };
                        let assistant_message = ChatMessage {
                            id: request_id_for_spawn.clone(),
                            role: "assistant".to_string(),
                            content: full_response,
                            timestamp: chrono::Local::now().to_rfc3339(),
                            files: None,
                            session_id: Some(session_id.clone()),
                            workspace_id: workspace_id_for_append.clone(),
                            model: None,
                        };
                        if let Err(e) = chat_session::append_message_to_session(
                            &session_id,
                            vec![user_message, assistant_message],
                            code_cli_for_append.clone(),
                            Some(task_id.clone()),
                        ) {
                            error!(
                                "Failed to append chat messages to session {}: {}",
                                session_id, e
                            );
                        }
                    }
                }
                Err(e) => {
                    if matches!(e, AppError::Cancelled(_)) {
                        debug!(
                            request_id = %request_id_for_spawn,
                            "AI streaming cancelled before completion"
                        );
                    } else {
                        error!("Failed to build AI response for streaming: {}", e);
                        let _ = emit_ai_response(
                            &app_handle_for_task,
                            &request_id_for_spawn,
                            &format!("[AI error] {}", e),
                            true,
                            None,
                            workspace_id_for_append.as_deref(),
                            None,
                        );
                    }
                }
            }
        })
    };

    let handle_entry = Arc::new(StreamingTaskHandle::new(join_handle, cancel_tx));

    {
        let state = app_handle.state::<AppState>();
        state
            .streaming_tasks
            .lock()
            .unwrap()
            .insert(request_id_for_task.clone(), handle_entry.clone());
    }

    let cleanup_handle = app_handle.clone();
    let request_id_for_cleanup = request_id_for_task.clone();
    let handle_entry_for_cleanup = handle_entry.clone();
    async_runtime::spawn(async move {
        if let Some(handle) = {
            let mut guard = handle_entry_for_cleanup.join_handle.lock().unwrap();
            guard.take()
        } {
            let _ = handle.await;
        }
        handle_entry_for_cleanup.cancel_tx.lock().unwrap().take();
        let app_state = cleanup_handle.state::<AppState>();
        let mut tasks = app_state.streaming_tasks.lock().unwrap();
        if let Some(current) = tasks.get(&request_id_for_cleanup) {
            if Arc::ptr_eq(current, &handle_entry_for_cleanup) {
                tasks.remove(&request_id_for_cleanup);
            }
        }
    });

    // 立即把 request_id 返回给前端，前端可用它在 Chat Messages Area 中关联消息
    Ok(request_id)
}


async fn read_line_if_available<R: AsyncRead + Unpin>(
    reader: &mut Option<BufReader<R>>,
    buf: &mut String,
) -> std::io::Result<usize> {
    if let Some(reader) = reader.as_mut() {
        buf.clear();
        reader.read_line(buf).await
    } else {
        Ok(0)
    }
}

fn build_direct_cli_args(backend: Option<&str>, user_args: &[String]) -> Vec<String> {
    let mut args = user_args.to_vec();
    match backend.map(|b| b.to_lowercase()) {
        Some(ref backend) if backend == "claude" => {
            if !has_cli_arg(&args, "-p") && !has_cli_arg(&args, "--print") {
                args.push("--print".to_string());
            }
            if !has_cli_arg(&args, "--output-format") {
                args.push("--output-format".to_string());
                args.push("text".to_string());
            }
        }
        Some(ref backend) if backend == "codex" => {
            if !has_codex_subcommand(&args) {
                args.insert(0, "exec".to_string());
            }
        }
        Some(ref backend) if backend == "gemini" => {
            if !has_cli_arg(&args, "--output-format") && !has_cli_arg(&args, "-o") {
                args.push("--output-format".to_string());
                args.push("text".to_string());
            }
        }
        _ => {}
    }
    args
}

struct DirectCliPlan {
    args: Vec<String>,
    task_id: Option<String>,
}

fn build_direct_cli_plan(
    backend: Option<&str>,
    user_args: &[String],
    resume_session_id: Option<&str>,
    code_cli_changed: Option<bool>,
) -> DirectCliPlan {
    let mut args = build_direct_cli_args(backend, user_args);
    let allow_resume = resume_session_id.filter(|_| !code_cli_changed.unwrap_or(false));
    let mut task_id: Option<String> = None;

    match backend.map(|b| b.to_lowercase()) {
        Some(ref backend) if backend == "claude" => {
            let existing_resume = get_flag_value(&args, "--resume")
                .or_else(|| get_flag_value(&args, "-r"));
            let existing_session_id = get_flag_value(&args, "--session-id");
            let has_continue = has_cli_arg(&args, "--continue") || has_cli_arg(&args, "-c");
            if let Some(id) = existing_session_id {
                task_id = Some(id);
            } else if let Some(id) = existing_resume {
                task_id = Some(id);
            } else if has_continue {
                task_id = Some("latest".to_string());
            } else if let Some(id) = allow_resume {
                args.push("--resume".to_string());
                args.push(id.to_string());
                task_id = Some(id.to_string());
            } else {
                let id = uuid::Uuid::new_v4().to_string();
                args.push("--session-id".to_string());
                args.push(id.clone());
                task_id = Some(id);
            }
        }
        Some(ref backend) if backend == "codex" => {
            let has_resume = has_codex_resume_subcommand(&args);
            if let Some(id) = allow_resume {
                if !has_codex_subcommand(&args) {
                    args.insert(0, "resume".to_string());
                } else if has_codex_exec_subcommand(&args) {
                    replace_codex_subcommand(&mut args, "resume");
                }
                let using_resume = has_codex_resume_subcommand(&args);
                if using_resume {
                    if id.eq_ignore_ascii_case("last") {
                        if !has_cli_arg(&args, "--last") {
                            args.push("--last".to_string());
                        }
                    } else {
                        args.push(id.to_string());
                    }
                }
                task_id = Some(id.to_string());
            } else if has_resume {
                task_id = Some("last".to_string());
            } else {
                task_id = Some("last".to_string());
            }
        }
        Some(ref backend) if backend == "gemini" => {
            let existing_resume = get_flag_value(&args, "--resume")
                .or_else(|| get_flag_value(&args, "-r"));
            if let Some(id) = existing_resume {
                task_id = Some(id);
            } else if let Some(id) = allow_resume {
                args.push("--resume".to_string());
                args.push(id.to_string());
                task_id = Some(id.to_string());
            } else {
                task_id = Some("latest".to_string());
            }
        }
        _ => {}
    }

    DirectCliPlan { args, task_id }
}

fn has_cli_arg(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name || arg.starts_with(&format!("{}=", name)))
}

fn has_codex_subcommand(args: &[String]) -> bool {
    let cmd = args.iter().find(|arg| !arg.starts_with('-'));
    if let Some(cmd) = cmd {
        matches!(
            cmd.as_str(),
            "exec"
                | "review"
                | "login"
                | "logout"
                | "mcp"
                | "mcp-server"
                | "app-server"
                | "completion"
                | "sandbox"
                | "apply"
                | "resume"
                | "cloud"
                | "features"
                | "help"
        )
    } else {
        false
    }
}

fn has_codex_exec_subcommand(args: &[String]) -> bool {
    let cmd = args.iter().find(|arg| !arg.starts_with('-'));
    matches!(cmd.map(|s| s.as_str()), Some("exec"))
}

fn has_codex_resume_subcommand(args: &[String]) -> bool {
    let cmd = args.iter().find(|arg| !arg.starts_with('-'));
    matches!(cmd.map(|s| s.as_str()), Some("resume"))
}

fn replace_codex_subcommand(args: &mut Vec<String>, replacement: &str) {
    if let Some((idx, _)) = args.iter().enumerate().find(|(_, arg)| !arg.starts_with('-')) {
        args[idx] = replacement.to_string();
    } else {
        args.insert(0, replacement.to_string());
    }
}

fn get_flag_value(args: &[String], name: &str) -> Option<String> {
    let flag = name.to_string();
    for (idx, arg) in args.iter().enumerate() {
        if arg == &flag {
            return args.get(idx + 1).cloned();
        }
        if let Some(rest) = arg.strip_prefix(&(flag.clone() + "=")) {
            return Some(rest.to_string());
        }
    }
    None
}

fn should_replace_task_id(current: Option<&str>, incoming: &str) -> bool {
    if incoming.trim().is_empty() {
        return false;
    }
    match current {
        None => true,
        Some(current) => matches!(current, "latest" | "last"),
    }
}

fn parse_cli_session_id(line: &str, backend: Option<&str>) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        if let Ok(value) = serde_json::from_str::<Value>(trimmed) {
            if let Some(id) = parse_session_id_from_json(&value, backend) {
                return Some(id);
            }
        }
    }

    let lowered = trimmed.to_lowercase();
    if lowered.starts_with("session id:") {
        return Some(trimmed["session id:".len()..].trim().to_string());
    }

    if let Some(idx) = lowered.find("session id:") {
        return Some(trimmed[idx + "session id:".len()..].trim().to_string());
    }

    None
}

fn parse_session_id_from_json(value: &Value, backend: Option<&str>) -> Option<String> {
    let backend = backend.unwrap_or("").to_lowercase();

    let session_id = value
        .get("session_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    if session_id.is_some() {
        return session_id;
    }

    if backend == "codex" {
        if let Some(thread_id) = value.get("thread_id").and_then(|v| v.as_str()) {
            return Some(thread_id.to_string());
        }
    }

    if let Some(value_type) = value.get("type").and_then(|v| v.as_str()) {
        if value_type == "thread.started" {
            if let Some(thread_id) = value.get("thread_id").and_then(|v| v.as_str()) {
                return Some(thread_id.to_string());
            }
        }
    }

    None
}

fn derive_backend_from_command(command: &str) -> Option<String> {
    let normalized = command.to_lowercase();
    if normalized.contains("claude") {
        Some("claude".to_string())
    } else if normalized.contains("codex") {
        Some("codex".to_string())
    } else if normalized.contains("gemini") {
        Some("gemini".to_string())
    } else {
        None
    }
}

/// Save clipboard image to a temporary file and return its absolute path
#[tauri::command]
pub async fn save_clipboard_image(
    app_handle: AppHandle,
    file_name: Option<String>,
    bytes: Vec<u8>,
) -> Result<String, String> {
    if bytes.is_empty() {
        return Err("Clipboard image data is empty".to_string());
    }
    let config = crate::core::app::get_config(app_handle.state::<AppState>());

    let mut dir= PathBuf::from(&config.app.data_dir);
    dir.push("clipboard-images");

    if let Err(err) = fs::create_dir_all(&dir) {
        return Err(format!("Failed to prepare clipboard directory: {}", err));
    }

    let generated_name = format!("codex-clipboard-{}.png", uuid::Uuid::new_v4());
    let final_name = file_name.unwrap_or(generated_name);
    let file_path = dir.join(final_name);

    fs::write(&file_path, bytes).map_err(|e| format!("Failed to write clipboard image: {}", e))?;

    Ok(file_path.to_string_lossy().to_string())
}

/// Cancel an in-flight streaming request by request_id
#[tauri::command]
pub async fn cancel_streaming_request(
    app_handle: AppHandle,
    request_id: String,
) -> Result<(), String> {
    let handle_entry = {
        let state = app_handle.state::<AppState>();
        let mut tasks = state.streaming_tasks.lock().unwrap();
        tasks.remove(&request_id)
    };

    if let Some(handle_entry) = handle_entry {
        if let Some(cancel_tx) = handle_entry.cancel_tx.lock().unwrap().take() {
            let _ = cancel_tx.send(());
        }
        Ok(())
    } else {
        Err("Streaming request not found or already finished".to_string())
    }
}

/// Execute command in terminal
#[tauri::command]
pub async fn execute_command(
    command: String,
    args: Vec<String>,
    cwd: Option<String>,
) -> Result<String, String> {
    info!("Executing command: {} {:?}", command, args);

    async_runtime::spawn_blocking(move || {
        let mut cmd = std::process::Command::new(&command);
        #[cfg(windows)]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd.args(&args);

        if let Some(dir) = cwd {
            cmd.current_dir(dir);
        }

        let output = cmd.output().map_err(|e| e.to_string())?;
        let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        if !stderr.is_empty() {
            error!("Command stderr: {}", stderr);
        }

        Ok::<String, String>(stdout)
    })
    .await
    .map_err(|e| format!("执行命令任务失败: {}", e))?
}

/// Execute a command in an existing terminal session
#[tauri::command]
pub async fn execute_terminal_command(
    state: State<'_, AppState>,
    session_id: String,
    shell: String,
    command: String,
) -> Result<String, String> {
    info!(
        "Executing terminal command in session {} with shell {}: {}",
        session_id, shell, command
    );

    state
        .terminal
        .execute_command(&session_id, &shell, &command)
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
    let date = chrono::Local::now().format("%Y-%m-%d");
    let path = {
        let cfg = state.config.lock().map_err(|e| e.to_string())?;
        let mut p = PathBuf::from(&cfg.logging.log_file_path);
        let filename = format!("{}.{}", cfg.logging.log_file_name, date);
        p.push(&filename);
        p
    };
    async_runtime::spawn_blocking(move || {
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

        Ok::<Vec<String>, String>(lines)
    })
    .await
    .map_err(|e| format!("读取日志任务失败: {}", e))?
}

/// Clear application logs by truncating the log file
#[tauri::command]
pub async fn clear_logs(state: State<'_, AppState>) -> Result<(), String> {
    info!("Clearing application logs");

    let path = {
        let cfg = state.config.lock().map_err(|e| e.to_string())?;
        let mut p = PathBuf::from(&cfg.logging.log_file_path);
        p.push(&cfg.logging.log_file_name);
        p
    };

    async_runtime::spawn_blocking(move || {
        if path.exists() {
            fs::write(&path, "").map_err(|e| e.to_string())?;
        }
        Ok::<(), String>(())
    })
    .await
    .map_err(|e| format!("清除日志任务失败: {}", e))?
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
