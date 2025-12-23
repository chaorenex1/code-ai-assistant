//! Database migrations bridge module
//!
//! Allows the main crate to reuse SeaORM migration files that live under
//! `src-tauri/migrations/migrations`.

#[path = "../../migrations/migrations/mod.rs"]
mod external_migrations;

pub use external_migrations::*;
