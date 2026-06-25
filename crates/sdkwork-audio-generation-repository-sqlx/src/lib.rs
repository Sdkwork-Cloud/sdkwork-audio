//! SDKWork Audio generation repository with SQLx
//!
//! This crate provides database access for audio generation tasks,
//! artifacts, and related entities.

mod bootstrap;

pub mod entities;
pub mod repositories;

pub use bootstrap::{
    bootstrap_audio_database, bootstrap_audio_database_from_env,
    connect_and_bootstrap_audio_database_from_env, connect_audio_database_pool_from_env,
    AudioDatabaseHost, AudioDatabasePool,
};

// Re-export main types
pub use entities::*;
pub use repositories::*;

// Re-export sdkwork-database types for convenience
pub use sdkwork_database_config::DatabaseConfig;
pub use sdkwork_database_sqlx::DatabasePool;

/// Create a new database pool using sdkwork-database configuration
pub async fn create_pool(config: &DatabaseConfig) -> Result<DatabasePool, sdkwork_database_sqlx::PoolError> {
    sdkwork_database_sqlx::create_pool_from_config(config.clone()).await
}

/// Create a new database pool from environment variables
pub async fn create_pool_from_env(service_name: &str) -> Result<Option<DatabasePool>, sdkwork_database_sqlx::PoolError> {
    sdkwork_database_sqlx::create_pool_from_env(service_name).await
}

/// Run migrations for SQLite dev pools. PostgreSQL uses application-root `database/` lifecycle.
pub async fn run_migrations(pool: &DatabasePool) -> Result<(), sqlx::Error> {
    match pool {
        DatabasePool::Postgres(_, _) => {
            bootstrap_audio_database(pool.clone())
                .await
                .map_err(|error| sqlx::Error::Configuration(error.into()))?;
            Ok(())
        }
        DatabasePool::Sqlite(sqlite_pool, _) => sqlx::migrate!("./migrations")
            .run(sqlite_pool)
            .await
            .map_err(sqlx::Error::from),
    }
}
