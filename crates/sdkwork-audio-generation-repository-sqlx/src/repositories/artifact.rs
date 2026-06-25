//! Artifact repository

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::entities::artifact::*;

/// Artifact repository trait
#[async_trait]
pub trait ArtifactRepository {
    async fn create(&self, request: CreateArtifactRequest) -> Result<AudioArtifact, sqlx::Error>;
    async fn get_by_id(&self, id: i64) -> Result<Option<AudioArtifact>, sqlx::Error>;
    async fn get_by_artifact_no(&self, artifact_no: &str) -> Result<Option<AudioArtifact>, sqlx::Error>;
    async fn list_by_task(&self, task_id: i64) -> Result<Vec<AudioArtifact>, sqlx::Error>;
    async fn list(&self, filter: ArtifactFilter, limit: i64, offset: i64) -> Result<ArtifactListResult, sqlx::Error>;
    async fn count(&self, filter: ArtifactFilter) -> Result<i64, sqlx::Error>;
}

/// SQLx implementation of ArtifactRepository
pub struct SqliteArtifactRepository {
    pool: SqlitePool,
}

impl SqliteArtifactRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ArtifactRepository for SqliteArtifactRepository {
    async fn create(&self, request: CreateArtifactRequest) -> Result<AudioArtifact, sqlx::Error> {
        let artifact_no = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let kind = request.kind.as_str();
        let status = "pending";

        let result = sqlx::query_as!(
            AudioArtifact,
            r#"
            INSERT INTO audio_audio_artifact (
                artifact_no, task_id, request_id, kind, artifact_type,
                title, voice_id, provider_code, provider_asset_id,
                artifact_index, format, mime_type, duration_seconds,
                transcript_text, translation_text, media_resource_json,
                status, created_at, updated_at, deleted, version
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#,
            artifact_no,
            request.task_id,
            request.request_id,
            kind,
            request.artifact_type,
            request.title,
            request.voice_id,
            request.provider_code,
            request.provider_asset_id,
            request.artifact_index,
            request.format,
            request.mime_type,
            request.duration_seconds,
            request.transcript_text,
            request.translation_text,
            request.media_resource_json,
            status,
            now,
            now,
            false,
            0
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<AudioArtifact>, sqlx::Error> {
        let artifact = sqlx::query_as!(
            AudioArtifact,
            r#"
            SELECT * FROM audio_audio_artifact
            WHERE id = ? AND deleted = FALSE
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(artifact)
    }

    async fn get_by_artifact_no(&self, artifact_no: &str) -> Result<Option<AudioArtifact>, sqlx::Error> {
        let artifact = sqlx::query_as!(
            AudioArtifact,
            r#"
            SELECT * FROM audio_audio_artifact
            WHERE artifact_no = ? AND deleted = FALSE
            "#,
            artifact_no
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(artifact)
    }

    async fn list_by_task(&self, task_id: i64) -> Result<Vec<AudioArtifact>, sqlx::Error> {
        let artifacts = sqlx::query_as!(
            AudioArtifact,
            r#"
            SELECT * FROM audio_audio_artifact
            WHERE task_id = ? AND deleted = FALSE
            ORDER BY artifact_index ASC
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(artifacts)
    }

    async fn list(&self, filter: ArtifactFilter, limit: i64, offset: i64) -> Result<ArtifactListResult, sqlx::Error> {
        let kind = filter.kind.map(|k| k.as_str().to_string());

        let artifacts = sqlx::query_as!(
            AudioArtifact,
            r#"
            SELECT * FROM audio_audio_artifact
            WHERE deleted = FALSE
                AND (? IS NULL OR task_id = ?)
                AND (? IS NULL OR kind = ?)
                AND (? IS NULL OR status = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            filter.task_id,
            filter.task_id,
            kind,
            kind,
            filter.status,
            filter.status,
            filter.created_after,
            filter.created_after,
            filter.created_before,
            filter.created_before,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let total = self.count(filter.clone()).await?;
        let has_more = (offset + limit) < total;
        let next_cursor = if has_more {
            Some((offset + limit).to_string())
        } else {
            None
        };

        Ok(ArtifactListResult {
            artifacts,
            total,
            has_more,
            next_cursor,
        })
    }

    async fn count(&self, filter: ArtifactFilter) -> Result<i64, sqlx::Error> {
        let kind = filter.kind.map(|k| k.as_str().to_string());

        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM audio_audio_artifact
            WHERE deleted = FALSE
                AND (? IS NULL OR task_id = ?)
                AND (? IS NULL OR kind = ?)
                AND (? IS NULL OR status = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            "#,
            filter.task_id,
            filter.task_id,
            kind,
            kind,
            filter.status,
            filter.status,
            filter.created_after,
            filter.created_after,
            filter.created_before,
            filter.created_before
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.unwrap_or(0))
    }
}
