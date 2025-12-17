//! Logging module
//!
//! This module handles application logging configuration.

use tauri::{App, Manager};
use time::{format_description::{BorrowedFormatItem, parse}, macros::format_description, UtcOffset};
use tracing_subscriber::fmt::time::UtcTime;

use anyhow::{Context, Result};
use tracing::Level;
use std::{fs, path::Path};
use tracing_subscriber::{
    fmt,
    layer::{Layer, SubscriberExt},
    prelude::*,
    registry::Registry,
    util::SubscriberInitExt,
    EnvFilter
};
use tracing_appender::{non_blocking, rolling::{RollingFileAppender, Rotation}};


fn build_timer() -> UtcTime<&'static [BorrowedFormatItem<'static>]> {
    // 等价：2025-12-18 12:34:56.123
    const FORMAT: &[BorrowedFormatItem<'static>] =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");

    UtcTime::new(FORMAT)
}


pub fn init_tracing(app: &mut App) -> Result<()> {
    let cfg = &app.state::<crate::config::AppConfig>().logging;
    let timer = build_timer();

    // 日志级别
    let env_filter = EnvFilter::new(&cfg.log_level);

    // === 文件输出 ===
    let max_size = (&cfg.log_file_rotation.log_file_max_size_mb * 1024 * 1024) as u64;
    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::DAILY)
        .max_log_files(cfg.log_file_rotation.log_file_max_backups as usize)
        .filename_prefix(&cfg.log_file_name)
        .build(&cfg.log_file_path.as_str())
        .context("Failed to create log file appender")?;

    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_timer(timer.clone())
        .with_writer(file_writer)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(false);

    // === 控制台输出 ===
    let stdout_layer = fmt::layer()
        .with_timer(timer)
        .with_writer(std::io::stdout)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_filter(if cfg.console { 
            env_filter.clone() 
        } else { 
            EnvFilter::new("off") 
        });

    // Use try_init to avoid panic if already initialized
    match tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .try_init() {
            Ok(_) => {
                // 防止日志丢失 - guard 必须在整个应用生命周期中保持
                std::mem::forget(guard);
                tracing::info!("Logging initialized successfully, log path: {}", cfg.log_file_path);
            },
            Err(e) => {
                // Subscriber already initialized - continue anyway but warn
                std::mem::forget(guard);
                eprintln!("Warning: Global tracing subscriber was already initialized: {:?}", e);
                eprintln!("Continuing with existing subscriber. Logs may not be written to file.");
            }
        }
    
    Ok(())
}


/// Log a message at debug level
pub fn debug(message: &str) {
    tracing::debug!("{}", message);
}

/// Log a message at info level
pub fn info(message: &str) {
    tracing::info!("{}", message);
}

/// Log a message at warn level
pub fn warn(message: &str) {
    tracing::warn!("{}", message);
}

/// Log a message at error level
pub fn error(message: &str) {
    tracing::error!("{}", message);
}

/// Log a message with structured fields
pub fn log_with_fields(level: Level, message: &str, _fields: &[(&str, &str)]) {
    match level {
        Level::DEBUG => tracing::debug!("{}", message),
        Level::INFO => tracing::info!("{}", message),
        Level::WARN => tracing::warn!("{}", message),
        Level::ERROR => tracing::error!("{}", message),
        Level::TRACE => tracing::trace!("{}", message),
    }
}

/// Create a span for tracing
pub fn create_span(name: &'static str) -> tracing::Span {
    tracing::info_span!("{}", name)
}