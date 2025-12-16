//! Code AI Assistant - Main entry point
//!
//! This is a desktop application with code editor, AI chat assistant,
//! CLI output and terminal functionality.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tracing::info;

mod config;
mod core;
mod database;
mod services;
#[path = "tauri/mod.rs"]
mod tauri_module;
mod utils;

/// Main entry point for the application
fn main() {
    tauri::Builder::default()
        // Register Tauri plugins
        .plugin(tauri_plugin_log::Builder::default().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            info!("Application already running with args: {:?}, cwd: {:?}", argv, cwd);

            // Focus the existing window
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_notification::init())

        // Register Tauri commands
        .invoke_handler(tauri::generate_handler![
            tauri_module::commands::read_file,
            tauri_module::commands::write_file,
            tauri_module::commands::list_files,
            tauri_module::commands::create_file,
            tauri_module::commands::delete_file,
            tauri_module::commands::rename_file,
            tauri_module::commands::create_directory,
            tauri_module::commands::list_directories,
            tauri_module::commands::delete_directory,
            tauri_module::commands::send_chat_message,
            tauri_module::commands::get_ai_models,
            tauri_module::commands::set_ai_model,
            tauri_module::commands::execute_command,
            tauri_module::commands::spawn_terminal,
            tauri_module::commands::kill_terminal,
            tauri_module::commands::get_settings,
            tauri_module::commands::save_settings,
            tauri_module::commands::reset_settings,
            tauri_module::commands::get_workspaces,
            tauri_module::commands::create_workspace,
            tauri_module::commands::switch_workspace,
            tauri_module::commands::delete_workspace,
            tauri_module::commands::get_system_info,
            tauri_module::commands::get_logs,
            tauri_module::commands::clear_logs,
        ])

        // Setup application state
        .setup(|app| {
            info!("Starting Code AI Assistant...");
            
            // Initialize application core
            core::app::init(app)?;

            // Initialize database connection
            database::connection::init(app)?;

            // Register event handlers
            tauri_module::events::register_event_handlers(app)?;

            info!("Application setup completed successfully");
            Ok(())
        })

        // Run the application
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}