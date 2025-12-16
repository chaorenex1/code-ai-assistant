//! Logging module
//!
//! This module handles application logging configuration.

use tracing::{info, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize application logging
pub fn init_logging() {
    // Create a filter that logs info level by default, but can be overridden by RUST_LOG
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // Configure the tracing subscriber
    tracing_subscriber::registry()
        .with(filter)
        .with(
            fmt::layer()
                .with_target(true)
                .with_level(true)
                .with_thread_ids(false)
                .with_thread_names(false)
                .compact(),
        )
        .init();

    info!("Logging initialized successfully");
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