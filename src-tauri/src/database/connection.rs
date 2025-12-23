//! Database connection module
//!
//! This module handles database connections and connection pooling.

use std::sync::Arc;
use tauri::{App, AppHandle, Manager, State};
use tracing::{error, info};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tokio::sync::Mutex;

use crate::config::schema::AppConfig;
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
    pub async fn get_connection(&self, database_url: &str, max_connections: u32, min_connections: u32) -> AppResult<DatabaseConnection> {
        let mut conn = self.connection.lock().await;

        if conn.is_none() {
            *conn = Some(Self::create_connection(database_url, max_connections, min_connections).await?);
        }

        Ok(conn.as_ref().unwrap().clone())
    }

    /// Create a new database connection
    async fn create_connection(database_url: &str, max_connections: u32, min_connections: u32) -> AppResult<DatabaseConnection> {
        let mut opt = ConnectOptions::new(database_url.to_string());
        opt.max_connections(max_connections)
            .min_connections(min_connections)
            .sqlx_logging(true)
            .sqlx_logging_level(tracing::log::LevelFilter::Debug);

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

    // Run migrations in background
    let app_handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        match get_db_connection(&app_handle).await {
            Ok(db) => {
                if let Err(e) = crate::database::migrations::run_migrations(&db).await {
                    error!("Failed to run database migrations: {}", e);
                } else {
                    info!("Database migrations completed successfully");
                }
            }
            Err(e) => {
                error!("Failed to get database connection for migrations: {}", e);
            }
        }
    });

    info!("Database initialized successfully");
    Ok(())
}

/// Get database connection from Tauri state with config
pub async fn get_db_connection(app_handle: &AppHandle) -> AppResult<DatabaseConnection> {
    let config = app_handle.state::<AppConfig>();
    let db_pool = app_handle.state::<DatabasePool>();
    
    db_pool.get_connection(
        &config.database.url,
        config.database.max_connections,
        config.database.min_connections
    ).await
}
