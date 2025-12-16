//! Code AI Assistant - Library crate
//!
//! This crate contains the core functionality for the Code AI Assistant application.

pub mod config;
pub mod core;
pub mod database;
pub mod services;
pub mod tauri;
pub mod utils;

/// Re-exports commonly used types
pub mod prelude {
    pub use crate::config::AppConfig;
    pub use crate::core::app::AppState;
    pub use crate::utils::error::{AppError, AppResult};
    pub use crate::utils::logging;
}