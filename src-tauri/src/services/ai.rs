//! AI Service module
//!
//! This module handles communication with AI models and CLI tools.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::oneshot;
use tokio::time::{sleep, Duration};
use tracing::{debug, info, warn};

use crate::utils::error::{AppError, AppResult};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CodeagentWrapperConfig {
    /// Optional explicit path to `codeagent-wrapper` binary.
    pub binary_path: Option<String>,
    /// Backend name: `codex` | `claude` | `gemini`.
    /// If omitted, will be derived from current model.
    pub backend: Option<String>,
    /// Working directory passed to wrapper (and used as process cwd).
    pub workdir: Option<String>,
    /// Skip CLI permission prompts (dangerous).
    pub skip_permissions: bool,
    /// Timeout in milliseconds (mapped to CODEX_TIMEOUT).
    pub timeout_ms: Option<u64>,

    /// Limit parallel workers (mapped to CODEAGENT_MAX_PARALLEL_WORKERS).
    pub max_parallel_workers: Option<u32>,
}

#[derive(Default)]
pub struct AiChatOptions {
    /// Selected code CLI name from UI (e.g. claude-cli/codex-cli/gemini-cli).
    pub code_cli: Option<String>,
    /// Resume session id for codeagent-wrapper.
    pub resume_session_id: Option<String>,
    /// Enable wrapper parallel mode (reads task config from stdin).
    pub parallel: bool,
    /// When using codex backend, optional model hint.
    /// Note: upstream `codeagent-wrapper` does not accept `--model`; we pass via env.
    pub codex_model: Option<String>,
    /// Workspace ID (used for context files).
    pub workspace_dir: Option<String>,
    /// Whether the code CLI has changed (used for session resuming).
    pub code_cli_changed: Option<bool>,
    /// Environment variables (mapped to CODEAGENT_ENV).
    pub env: Vec<(String, String)>,
    /// Cancellation signal used to terminate codeagent-wrapper.
    pub cancel_rx: Option<oneshot::Receiver<()>>,
}

impl fmt::Debug for AiChatOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AiChatOptions")
            .field("code_cli", &self.code_cli)
            .field("resume_session_id", &self.resume_session_id)
            .field("parallel", &self.parallel)
            .field("codex_model", &self.codex_model)
            .field("workspace_dir", &self.workspace_dir)
            .field("code_cli_changed", &self.code_cli_changed)
            .field("env_len", &self.env.len())
            .field("has_cancel_rx", &self.cancel_rx.is_some())
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct AiMessageResult {
    pub message: String,
    pub codeagent_session_id: Option<String>,
}

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

/// AI Service for managing AI models and sending messages
#[derive(Debug)]
pub struct AiService {
    /// Available AI models
    models: Vec<AiModel>,
    /// Current model
    current_model: Option<String>,
    /// HTTP client
    _client: reqwest::Client,

    /// codeagent-wrapper integration
    codeagent: CodeagentWrapperConfig,
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
            current_model: Some("claude-3-5-sonnet".to_string()),
            _client: reqwest::Client::new(),
            codeagent: CodeagentWrapperConfig {
                binary_path: None,
                backend: None,
                workdir: None,
                skip_permissions: false,
                timeout_ms: None,
                max_parallel_workers: None,
            },
        }
    }

    pub fn set_codeagent_config(&mut self, config: CodeagentWrapperConfig) {
        self.codeagent = config;
    }

    pub fn get_codeagent_config(&self) -> CodeagentWrapperConfig {
        self.codeagent.clone()
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
        Ok(
            self
                .send_message_with_options(message, _context_files, AiChatOptions::default())
                .await?
                .message,
        )
    }

    pub async fn send_message_with_options(
        &self,
        message: &str,
        _context_files: Option<Vec<String>>,
        options: AiChatOptions,
    ) -> AppResult<AiMessageResult> {
        debug!("Sending message to AI: {} {:?}", message, options);

        // Prefer codeagent-wrapper when available.
        // IMPORTANT: UI-selected `code_cli` should take precedence over any persisted config,
        // otherwise users see a mismatch (e.g. code_cli=gemini-cli but backend=claude).
        let backend = options
            .code_cli
            .as_deref()
            .and_then(Self::derive_backend_from_code_cli)
            .or_else(|| self.codeagent.backend.clone())
            .unwrap_or_else(|| self.derive_backend_from_current_model());
        let workdir = options.workspace_dir
                    .as_deref()
                    .map(|w| w.to_string())
                    .or_else(|| self.get_codeagent_config().workdir.clone())
                    .unwrap_or_else(|| ".".to_string());

        let task = Self::build_task_with_context(message, _context_files.as_deref());

        let result = self
            .run_codeagent_wrapper(CodeagentRunSpec {
                task,
                backend,
                workdir,
                skip_permissions: self.codeagent.skip_permissions,
                timeout_ms: self.codeagent.timeout_ms,
                max_parallel_workers: self.codeagent.max_parallel_workers,
                binary_path: self.codeagent.binary_path.clone(),
                resume_session_id: options.resume_session_id,
                parallel: options.parallel,
                codex_model: options.codex_model,
                env: options.env,
                code_cli_changed: options.code_cli_changed,
                cancel_rx: options.cancel_rx,
            })
            .await?;

        Ok(AiMessageResult {
            message: result.message,
            codeagent_session_id: result.session_id,
        })
    }

    /// Add a new model
    pub fn add_model(&mut self, model: AiModel) {
        self.models.push(model);
    }

    /// Remove a model
    pub fn remove_model(&mut self, name: &str) {
        self.models.retain(|m| m.name != name);
    }

    fn derive_backend_from_current_model(&self) -> String {
        let model = self.current_model.as_deref().unwrap_or("");
        let m = model.to_lowercase();
        if m.contains("claude") {
            "claude".to_string()
        } else if m.contains("gemini") {
            "gemini".to_string()
        } else {
            // Fallback: treat OpenAI-like model ids as codex CLI backend.
            "codex".to_string()
        }
    }

    pub(crate) fn derive_backend_from_code_cli(code_cli: &str) -> Option<String> {
        let v = code_cli.trim().to_lowercase();
        if v.is_empty() {
            return None;
        }
        if v.contains("claude") {
            Some("claude".to_string())
        } else if v.contains("gemini") {
            Some("gemini".to_string())
        } else if v.contains("codex") {
            Some("codex".to_string())
        } else {
            None
        }
    }

    pub(crate) fn build_task_with_context(
        message: &str,
        context_files: Option<&[String]>,
    ) -> String {
        if let Some(files) = context_files {
            if !files.is_empty() {
                let mut segments: Vec<String> = Vec::new();
                for file_path in files {
                    match fs::read_to_string(file_path) {
                        Ok(content) => {
                            segments.push(format!("@{}\n{}", file_path, content));
                        }
                        Err(err) => {
                            warn!(
                                path = %file_path,
                                error = %err,
                                "Failed to read context file"
                            );
                        }
                    }
                }

                if !segments.is_empty() {
                    let mut prefix = segments.join("\n");
                    prefix.push_str("\n\n");
                    return format!("{}{}", prefix, message);
                }
            }
        }

        message.to_string()
    }

    fn is_executable_file(path: &Path) -> bool {
        if !path.is_file() {
            return false;
        }

        #[cfg(target_os = "windows")]
        {
            return path
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e.eq_ignore_ascii_case("exe"))
                .unwrap_or(false);
        }

        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = std::fs::metadata(path) {
                return (meta.permissions().mode() & 0o111) != 0;
            }
            false
        }
    }

    fn which_in_path(program: &str) -> Option<PathBuf> {
        let path_var = std::env::var_os("PATH")?;
        for dir in std::env::split_paths(&path_var) {
            let candidate = dir.join(program);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        None
    }

    fn find_codeagent_wrapper(explicit_path: Option<String>) -> AppResult<PathBuf> {
        if let Some(p) = explicit_path {
            let pb = PathBuf::from(p);
            if pb.exists() {
                return Ok(pb);
            }
            return Err(AppError::AiServiceError(
                "指定的 codeagent-wrapper 路径不存在".to_string(),
            ));
        }

        #[cfg(target_os = "windows")]
        let program_name = "codeagent-wrapper.exe";
        #[cfg(not(target_os = "windows"))]
        let program_name = "codeagent-wrapper";

        if let Some(p) = Self::which_in_path(program_name) {
            return Ok(p);
        }

        if let Some(home) = dirs::home_dir() {
            // myclaude README: Windows installer places it under %USERPROFILE%\bin
            let candidates = [
                home.join("bin").join(program_name),
                home.join(".claude").join("bin").join(program_name),
            ];
            for c in candidates {
                if c.exists() {
                    return Ok(c);
                }
            }
        }

        // Dev convenience: `pnpm fetch:codeagent-wrapper` downloads to `src-tauri/bin/`.
        let dev_candidate = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("bin")
            .join(program_name);
        if dev_candidate.exists() {
            return Ok(dev_candidate);
        }

        Err(AppError::AiServiceError(
            "未找到 codeagent-wrapper。请先安装 myclaude（或将 codeagent-wrapper 放入 PATH / $HOME/bin / $HOME/.claude/bin），或在配置中设置 binary_path。\n\n开发模式可运行：pnpm fetch:codeagent-wrapper".to_string(),
        ))
    }

    async fn run_codeagent_wrapper(&self, spec: CodeagentRunSpec) -> AppResult<CodeagentRunResult> {
        let bin = Self::find_codeagent_wrapper(spec.binary_path)?;
        if !Self::is_executable_file(&bin) {
            return Err(AppError::AiServiceError(format!(
                "codeagent-wrapper 不是可执行文件: {}",
                bin.display()
            )));
        }

        let mut args: Vec<String> = Vec::new();
        if !spec.backend.trim().is_empty() {
            args.push("--backend".to_string());
            args.push(spec.backend.trim().to_string());
        }

        if spec.parallel {
            args.push("--parallel".to_string());
        }

        if spec.skip_permissions {
            // Wrapper supports both --skip-permissions and --dangerously-skip-permissions.
            args.push("--dangerously-skip-permissions".to_string());
        }

        if let Some(resume) = spec.resume_session_id.as_deref() {
                let resume = resume.trim();
                if !resume.is_empty() && !spec.code_cli_changed.unwrap_or(false) {
                    args.push("resume".to_string());
                    args.push(resume.to_string());
                }
            }
        if spec.parallel {
                        // Parallel mode reads task config from stdin and forbids extra args.
        } else {
            // Always use stdin mode to avoid shell quoting issues.
            args.push(spec.task.to_string());
            args.push(spec.workdir.clone());
        }

        let mut cmd = Command::new(&bin);
        #[cfg(windows)]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }
        cmd.args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(&spec.workdir);

        if let Some(timeout_ms) = spec.timeout_ms {
            // myclaude wrapper reads CODEX_TIMEOUT in milliseconds.
            cmd.env("CODEX_TIMEOUT", timeout_ms.to_string());
        }
        if spec.skip_permissions {
            // Also enable via env for wrapper default.
            cmd.env("CODEAGENT_SKIP_PERMISSIONS", "1");
        }

        if let Some(max_workers) = spec.max_parallel_workers {
            cmd.env("CODEAGENT_MAX_PARALLEL_WORKERS", max_workers.to_string());
        }

        // Upstream wrapper currently does not accept `--model` for codex.
        // We pass a best-effort hint via env for codex CLI implementations.
        if spec.backend.trim().eq_ignore_ascii_case("codex") {
            if let Some(m) = spec.codex_model.as_deref() {
                let m = m.trim();
                if !m.is_empty() {
                    cmd.env("CODEX_MODEL", m);
                }
            }
        }
        

        debug!(
            codeagent_bin = %bin.display(),
            workdir = %spec.workdir,
            backend = %spec.backend,
            parallel = spec.parallel,
            skip_permissions = spec.skip_permissions,
            timeout_ms = ?spec.timeout_ms,
            max_parallel_workers = ?spec.max_parallel_workers,
            resume_session_id = ?spec.resume_session_id,
            "Executing codeagent-wrapper"
        );
        debug!(args = ?args, "codeagent-wrapper args");
        if let Some(timeout_ms) = spec.timeout_ms {
            debug!(CODEX_TIMEOUT = timeout_ms, "codeagent-wrapper env");
        }
        if spec.skip_permissions {
            debug!(CODEAGENT_SKIP_PERMISSIONS = "1", "codeagent-wrapper env");
        }
        if let Some(max_workers) = spec.max_parallel_workers {
            debug!(CODEAGENT_MAX_PARALLEL_WORKERS = max_workers, "codeagent-wrapper env");
        }
        if spec.backend.trim().eq_ignore_ascii_case("codex") {
            if let Some(m) = spec.codex_model.as_deref().map(str::trim).filter(|s| !s.is_empty()) {
                debug!(CODEX_MODEL = %m, "codeagent-wrapper env");
            }
        }
        if !spec.env.is_empty() {
            for (key, value) in &spec.env {
                cmd.env(key, value);
            }
        }

        let mut child = cmd.spawn().map_err(|e| {
            AppError::AiServiceError(format!(
                "启动 codeagent-wrapper 失败: {} (bin={})",
                e,
                bin.display()
            ))
        })?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(spec.task.as_bytes())
                .await
                .map_err(|e| AppError::AiServiceError(format!("写入 codeagent-wrapper stdin 失败: {}", e)))?;
        }

        let stdout_task = child.stdout.take().map(|mut stdout_pipe| {
            tokio::spawn(async move {
                let mut buf = Vec::new();
                stdout_pipe
                    .read_to_end(&mut buf)
                    .await
                    .map_err(|e| AppError::AiServiceError(format!("读取 codeagent-wrapper stdout 失败: {}", e)))?;
                Ok(buf)
            })
        });
        let stderr_task = child.stderr.take().map(|mut stderr_pipe| {
            tokio::spawn(async move {
                let mut buf = Vec::new();
                stderr_pipe
                    .read_to_end(&mut buf)
                    .await
                    .map_err(|e| AppError::AiServiceError(format!("读取 codeagent-wrapper stderr 失败: {}", e)))?;
                Ok(buf)
            })
        });

        let mut cancel_rx = spec.cancel_rx;
        let exit_status = loop {
            if let Some(status) = child
                .try_wait()
                .map_err(|e| AppError::AiServiceError(format!("轮询 codeagent-wrapper 状态失败: {}", e)))?
            {
                break status;
            }

            if let Some(cancel_fut) = cancel_rx.as_mut() {
                tokio::select! {
                    _ = cancel_fut => {
                        if let Err(e) = child.kill().await {
                            warn!(error = %e, "Failed to kill codeagent-wrapper after cancellation");
                        }
                        return Err(AppError::Cancelled("codeagent-wrapper cancelled".to_string()));
                    }
                    _ = sleep(Duration::from_millis(50)) => {}
                }
            } else {
                sleep(Duration::from_millis(50)).await;
            }
        };

        let stdout_bytes = collect_process_output(stdout_task).await?;
        let stderr_bytes = collect_process_output(stderr_task).await?;
        let stdout = String::from_utf8_lossy(&stdout_bytes).into_owned();
        let stderr = String::from_utf8_lossy(&stderr_bytes).into_owned();
        let exit_code = exit_status.code().unwrap_or(-1);

        debug!(
            exit_code,
            stdout_len = stdout.len(),
            stderr_len = stderr.len(),
            "codeagent-wrapper finished"
        );

        if exit_code != 0 {
            let stderr_tail = tail_snippet(&stderr, 4000);
            let stdout_tail = tail_snippet(&stdout, 4000);
            warn!(
                exit_code,
                stderr_tail = %stderr_tail,
                stdout_tail = %stdout_tail,
                "codeagent-wrapper failed"
            );
            return Err(AppError::AiServiceError(format!(
                "codeagent-wrapper 退出码 {}。stderr: {}",
                exit_code,
                stderr.trim()
            )));
        }

        let (message, session_id) = parse_codeagent_stdout(&stdout);
        debug!(
            parsed_session_id = ?session_id,
            message_len = message.len(),
            stdout_tail = %tail_snippet(&stdout, 1000),
            "parsed codeagent-wrapper stdout"
        );
        if message.trim().is_empty() {
            return Err(AppError::AiServiceError(format!(
                "codeagent-wrapper 未返回有效消息。stderr: {}",
                stderr.trim()
            )));
        }

        Ok(CodeagentRunResult {
            message,
            session_id,
            raw_stdout: stdout,
            raw_stderr: stderr,
            exit_code,
        })
    }
}

async fn collect_process_output(
    task: Option<tokio::task::JoinHandle<AppResult<Vec<u8>>>>,
) -> AppResult<Vec<u8>> {
    if let Some(handle) = task {
        match handle.await {
            Ok(res) => res,
            Err(e) => Err(AppError::AiServiceError(format!(
                "收集子进程输出失败: {}",
                e
            ))),
        }
    } else {
        Ok(Vec::new())
    }
}

struct CodeagentRunSpec {
    task: String,
    backend: String,
    workdir: String,
    skip_permissions: bool,
    timeout_ms: Option<u64>,
    max_parallel_workers: Option<u32>,
    binary_path: Option<String>,
    resume_session_id: Option<String>,
    parallel: bool,
    codex_model: Option<String>,
    env: Vec<(String, String)>,
    code_cli_changed: Option<bool>,
    cancel_rx: Option<oneshot::Receiver<()>>,
}

#[derive(Debug, Clone)]
struct CodeagentRunResult {
    message: String,
    #[allow(dead_code)]
    session_id: Option<String>,
    #[allow(dead_code)]
    raw_stdout: String,
    #[allow(dead_code)]
    raw_stderr: String,
    #[allow(dead_code)]
    exit_code: i32,
}

fn parse_codeagent_stdout(stdout: &str) -> (String, Option<String>) {
    // Wrapper prints:
    // <message>\n
    // ---\n
    // SESSION_ID: <id>\n
    // On Windows, output may contain CRLF; normalize to LF for parsing.
    let normalized = stdout.replace("\r\n", "\n");

    // Preferred exact marker
    let marker = "\n---\nSESSION_ID:";
    if let Some(idx) = normalized.rfind(marker) {
        let message = normalized[..idx].trim().to_string();
        let tail = &normalized[idx + marker.len()..];
        let session_id = tail
            .lines()
            .next()
            .map(|s| s.trim().trim_start_matches(':').trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        return (message, session_id);
    }

    // Fallback: some versions may omit the leading newline before ---
    let marker2 = "---\nSESSION_ID:";
    if let Some(idx) = normalized.rfind(marker2) {
        let message = normalized[..idx].trim().to_string();
        let tail = &normalized[idx + marker2.len()..];
        let session_id = tail
            .lines()
            .next()
            .map(|s| s.trim().trim_start_matches(':').trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        return (message, session_id);
    }

    // Fallback: at least try to parse SESSION_ID even if delimiter formatting changes.
    if let Some(idx) = normalized.rfind("SESSION_ID:") {
        let before = &normalized[..idx];
        let after = &normalized[idx + "SESSION_ID:".len()..];
        let session_id = after
            .lines()
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string());
        let message = before
            .trim_end_matches('-')
            .trim()
            .to_string();
        return (message, session_id);
    }

    (normalized.trim().to_string(), None)
}

fn tail_snippet(s: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let normalized = s.replace("\r\n", "\n");
    let mut out = if normalized.chars().count() <= max_chars {
        normalized
    } else {
        let start = normalized
            .char_indices()
            .rev()
            .nth(max_chars)
            .map(|(i, _)| i)
            .unwrap_or(0);
        format!("…{}", &normalized[start..])
    };
    // Avoid huge single-line log entries.
    if out.len() > max_chars + 2 {
        out.truncate(max_chars + 2);
    }
    out
}

impl Default for AiService {
    fn default() -> Self {
        Self::new()
    }
}
