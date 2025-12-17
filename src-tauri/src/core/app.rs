//! Application core module
//!
//! This module contains the core application logic and state management.

use std::sync::{Arc, Mutex};
use tauri::{App, AppHandle, Manager, State};

use crate::utils::error::AppResult;
use crate::config::schema::AppConfig;
use crate::services::terminal::TerminalService;

/// Application state shared across the application
#[derive(Debug)]
pub struct AppState {
    /// Application handle
    pub app_handle: AppHandle,
    /// Application configuration (wrapped in Mutex for modification)
    pub config: Mutex<AppConfig>,
    /// Database connection pool
    pub db_pool: Arc<crate::database::connection::DatabasePool>,
    /// Terminal service for managing terminal sessions
    pub terminal: TerminalService,
}

impl AppState {
    /// Create a new application state
    pub fn new(
        app_handle: AppHandle,
        config: AppConfig,
        db_pool: Arc<crate::database::connection::DatabasePool>,
    ) -> Self {
        Self {
            app_handle,
            config: Mutex::new(config),
            db_pool,
            terminal: TerminalService::new(),
        }
    }
}

/// Initialize the application core
pub fn init(app: &mut App) -> AppResult<()> {
    // Load application configuration
    let config = crate::config::loader::load_config()?;
    // initialize data directory
    crate::utils::fs::init_dir(&config.app.data_dir)?;
    // Store configuration in Tauri state
    app.manage(config.clone());

    // Create application state
    let mut app_state = AppState::new(
        app.handle().clone(),
        config,
        Arc::new(crate::database::connection::DatabasePool::new()),
    );

    // Use configured default shell for terminal service if provided
    {
        let cfg = app_state.config.lock().unwrap();
        app_state
            .terminal
            .set_default_shell(cfg.cli.default_shell.clone());
    }

    // Store application state in Tauri state
    app.manage(app_state);

    Ok(())
}

/// Get application state from Tauri state
pub fn get_app_state(state: State<'_, AppState>) -> &AppState {
    state.inner()
}

/// Get application handle from Tauri state
pub fn get_app_handle(state: State<'_, AppState>) -> AppHandle {
    state.inner().app_handle.clone()
}

/// Get application configuration from Tauri state
pub fn get_config(state: State<'_, AppState>) -> AppConfig {
    state.inner().config.lock().unwrap().clone()
}

/// Get database connection pool from Tauri state
pub fn get_db_pool(state: State<'_, AppState>) -> Arc<crate::database::connection::DatabasePool> {
    state.inner().db_pool.clone()
}