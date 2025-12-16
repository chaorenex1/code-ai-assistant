//! Database connection module
//!
//! This module handles database connections and connection pooling.

use std::sync::Arc;
use tauri::{App, Manager};
use tracing::{error, info};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::Mutex;

use crate::utils::error::{AppError, AppResult};

/// Database connection pool wrapper
#[derive(Debug, Clone)]
pub struct DatabasePool {
    /// Database connection
    connection: Arc<Mutex<Option<DatabaseConnection>>>,
}

impl DatabasePool {
    /// Create a new database pool
    pub fn new() -> Self {
        Self {
            connection: Arc::new(Mutex::new(None)),
        }
    }

    /// Get database connection
    pub async fn get_connection(&self) -> AppResult<DatabaseConnection> {
        let mut conn = self.connection.lock().await;

        if conn.is_none() {
            *conn = Some(Self::create_connection().await?);
        }

        Ok(conn.as_ref().unwrap().clone())
    }

    /// Create a new database connection
    async fn create_connection() -> AppResult<DatabaseConnection> {
        // TODO: Load database URL from configuration
        let database_url = "sqlite://data/app.db?mode=rwc";

        let mut opt = ConnectOptions::new(database_url.to_string());
        opt.max_connections(10)
            .min_connections(1)
            .sqlx_logging(true)
            .sqlx_logging_level(tracing::log::LevelFilter::Info);

        info!("Connecting to database: {}", database_url);

        match Database::connect(opt).await {
            Ok(conn) => {
                info!("Database connection established successfully");
                Ok(conn)
            }
            Err(err) => {
                error!("Failed to connect to database: {}", err);
                Err(AppError::DatabaseError(err.to_string()))
            }
        }
    }

    /// Close database connection
    pub async fn close(&self) -> AppResult<()> {
        let mut conn = self.connection.lock().await;

        if let Some(connection) = conn.take() {
            info!("Closing database connection...");
            connection.close().await?;
            info!("Database connection closed successfully");
        }

        Ok(())
    }
}

/// Initialize database connection
pub fn init(app: &mut App) -> AppResult<()> {
    info!("Initializing database...");

    // Store database pool in Tauri state
    let db_pool = DatabasePool::new();
    app.manage(db_pool);

    info!("Database initialized successfully");
    Ok(())
}