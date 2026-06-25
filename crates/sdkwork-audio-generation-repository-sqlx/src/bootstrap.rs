//! SDKWork Audio database pool bootstrap via `sdkwork-database`.

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool, PoolError};

pub use sdkwork_audio_database_host::{
    bootstrap_audio_database, bootstrap_audio_database_from_env, AudioDatabaseHost,
};

pub type AudioDatabasePool = DatabasePool;

pub async fn connect_audio_database_pool_from_env() -> Result<AudioDatabasePool, PoolError> {
    let config = DatabaseConfig::from_env("AUDIO")?;
    create_pool_from_config(config).await
}

pub async fn connect_and_bootstrap_audio_database_from_env() -> Result<AudioDatabaseHost, String> {
    let pool = connect_audio_database_pool_from_env()
        .await
        .map_err(|error| error.to_string())?;
    bootstrap_audio_database(pool).await
}
