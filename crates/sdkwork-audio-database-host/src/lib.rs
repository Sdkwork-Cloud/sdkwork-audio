use std::path::PathBuf;
use std::sync::Arc;

use sdkwork_database_config::DatabaseConfig;
use sdkwork_database_lifecycle::{lifecycle_options_from_env, LifecycleOrchestrator};
use sdkwork_database_spi::{DatabaseAssetProvider, DatabaseManifest, DefaultDatabaseModule};
use sdkwork_database_sqlx::{create_pool_from_config, DatabasePool};

pub struct AudioDatabaseHost {
    pool: DatabasePool,
    module: Arc<DefaultDatabaseModule>,
}

impl AudioDatabaseHost {
    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }

    pub fn module(&self) -> Arc<DefaultDatabaseModule> {
        self.module.clone()
    }
}

pub async fn bootstrap_audio_database(pool: DatabasePool) -> Result<AudioDatabaseHost, String> {
    let app_root = resolve_app_root();
    let module = Arc::new(
        DefaultDatabaseModule::from_app_root(&app_root)
            .map_err(|error| format!("load audio database module failed: {error}"))?,
    );
    let manifest = DatabaseManifest::from_file(module.manifest_path())
        .map_err(|error| format!("read audio database manifest failed: {error}"))?;
    let options = lifecycle_options_from_env("AUDIO", &manifest);
    let orchestrator =
        LifecycleOrchestrator::new(pool.clone(), module.clone()).with_applied_by("sdkwork-audio");

    orchestrator
        .init()
        .await
        .map_err(|error| format!("audio database init failed: {error}"))?;

    if options.auto_migrate {
        orchestrator
            .migrate()
            .await
            .map_err(|error| format!("audio database migrate failed: {error}"))?;
    }

    Ok(AudioDatabaseHost { pool, module })
}

pub async fn bootstrap_audio_database_from_env() -> Result<AudioDatabaseHost, String> {
    let _ = dotenvy::dotenv();
    let config = DatabaseConfig::from_env("AUDIO")
        .map_err(|error| format!("read audio database config failed: {error}"))?;
    let pool = create_pool_from_config(config)
        .await
        .map_err(|error| format!("create audio database pool failed: {error}"))?;
    bootstrap_audio_database(pool).await
}

fn resolve_app_root() -> PathBuf {
    std::env::var("SDKWORK_AUDIO_APP_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .canonicalize()
                .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../.."))
        })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    use sdkwork_database_config::{DatabaseConfig, DatabaseEngine};
    use sdkwork_database_sqlx::{DatabasePool, PoolContext};
    use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

    use super::bootstrap_audio_database;

    #[tokio::test]
    #[ignore = "requires SDKWORK_AUDIO_TEST_DATABASE_URL"]
    async fn postgres_lifecycle_bootstrap_executes_complete_audio_baseline() {
        let database_url = std::env::var("SDKWORK_AUDIO_TEST_DATABASE_URL")
            .expect("SDKWORK_AUDIO_TEST_DATABASE_URL must point to a PostgreSQL test database");
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time must be after the Unix epoch")
            .as_nanos();
        let schema = format!("sdkwork_audio_test_{}_{}", std::process::id(), unique);
        let quoted_schema = format!("\"{schema}\"");
        let previous_schema = std::env::var_os("SDKWORK_AUDIO_DATABASE_SCHEMA");
        std::env::set_var("SDKWORK_AUDIO_DATABASE_SCHEMA", &schema);

        let admin_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("connect to PostgreSQL test database");
        sqlx::query(&format!("CREATE SCHEMA {quoted_schema}"))
            .execute(&admin_pool)
            .await
            .expect("create isolated PostgreSQL test schema");

        let connect_options = PgConnectOptions::from_str(&database_url)
            .expect("parse PostgreSQL test database URL")
            .options([("search_path", schema.as_str())]);
        let scoped_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect_with(connect_options)
            .await
            .expect("connect to isolated PostgreSQL test schema");
        let pool = DatabasePool::Postgres(
            scoped_pool,
            PoolContext {
                config: DatabaseConfig {
                    engine: DatabaseEngine::Postgres,
                    url: database_url,
                    max_connections: 1,
                    min_connections: 0,
                    ..Default::default()
                },
            },
        );

        let bootstrap_result = bootstrap_audio_database(pool.clone()).await;
        let table_count = match &bootstrap_result {
            Ok(_) => {
                sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = $1 AND table_name LIKE 'audio_%'",
                )
                .bind(&schema)
                .fetch_one(pool.as_postgres().expect("PostgreSQL pool"))
                .await
                .expect("count materialized Audio tables")
            }
            Err(_) => 0,
        };

        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {quoted_schema} CASCADE"))
            .execute(&admin_pool)
            .await
            .expect("drop isolated PostgreSQL test schema");
        admin_pool.close().await;
        match previous_schema {
            Some(value) => std::env::set_var("SDKWORK_AUDIO_DATABASE_SCHEMA", value),
            None => std::env::remove_var("SDKWORK_AUDIO_DATABASE_SCHEMA"),
        }

        bootstrap_result.expect("Audio PostgreSQL lifecycle bootstrap must succeed");
        assert_eq!(table_count, 14, "all registered Audio tables must exist");
    }
}
