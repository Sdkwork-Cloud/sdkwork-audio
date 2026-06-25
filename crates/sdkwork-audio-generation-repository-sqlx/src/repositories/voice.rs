//! Voice repository

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::entities::voice::*;

/// Voice repository trait
#[async_trait]
pub trait VoiceRepository {
    async fn create(&self, request: CreateVoiceRequest) -> Result<AudioVoice, sqlx::Error>;
    async fn get_by_id(&self, id: i64) -> Result<Option<AudioVoice>, sqlx::Error>;
    async fn get_by_voice_no(&self, voice_no: &str) -> Result<Option<AudioVoice>, sqlx::Error>;
    async fn update(&self, id: i64, request: UpdateVoiceRequest) -> Result<AudioVoice, sqlx::Error>;
    async fn list(&self, filter: VoiceFilter, limit: i64, offset: i64) -> Result<VoiceListResult, sqlx::Error>;
    async fn count(&self, filter: VoiceFilter) -> Result<i64, sqlx::Error>;
}

/// SQLx implementation of VoiceRepository
pub struct SqliteVoiceRepository {
    pool: SqlitePool,
}

impl SqliteVoiceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VoiceRepository for SqliteVoiceRepository {
    async fn create(&self, request: CreateVoiceRequest) -> Result<AudioVoice, sqlx::Error> {
        let voice_no = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let voice_type = request.voice_type.as_str();
        let status = VoiceStatus::Active.as_str();

        let result = sqlx::query_as!(
            AudioVoice,
            r#"
            INSERT INTO audio_voice (
                voice_no, tenant_id, organization_id, user_id, name,
                description, language, gender, voice_type,
                provider_code, provider_voice_id, provider_config_json,
                clone_reference_url, clone_parameters_json,
                status, is_public, usage_count, quality_score,
                created_at, updated_at, deleted, version
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#,
            voice_no,
            request.tenant_id,
            request.organization_id,
            request.user_id,
            request.name,
            request.description,
            request.language,
            request.gender,
            voice_type,
            request.provider_code,
            request.provider_voice_id,
            request.provider_config_json,
            request.clone_reference_url,
            request.clone_parameters_json,
            status,
            request.is_public,
            0,
            None,
            now,
            now,
            false,
            0
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<AudioVoice>, sqlx::Error> {
        let voice = sqlx::query_as!(
            AudioVoice,
            r#"
            SELECT * FROM audio_voice
            WHERE id = ? AND deleted = FALSE
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(voice)
    }

    async fn get_by_voice_no(&self, voice_no: &str) -> Result<Option<AudioVoice>, sqlx::Error> {
        let voice = sqlx::query_as!(
            AudioVoice,
            r#"
            SELECT * FROM audio_voice
            WHERE voice_no = ? AND deleted = FALSE
            "#,
            voice_no
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(voice)
    }

    async fn update(&self, id: i64, request: UpdateVoiceRequest) -> Result<AudioVoice, sqlx::Error> {
        let now = chrono::Utc::now();
        let voice_type = request.voice_type.map(|vt| vt.as_str().to_string());
        let status = request.status.map(|s| s.as_str().to_string());

        let result = sqlx::query_as!(
            AudioVoice,
            r#"
            UPDATE audio_voice SET
                name = COALESCE(?, name),
                description = COALESCE(?, description),
                language = COALESCE(?, language),
                gender = COALESCE(?, gender),
                voice_type = COALESCE(?, voice_type),
                provider_code = COALESCE(?, provider_code),
                provider_voice_id = COALESCE(?, provider_voice_id),
                provider_config_json = COALESCE(?, provider_config_json),
                clone_reference_url = COALESCE(?, clone_reference_url),
                clone_parameters_json = COALESCE(?, clone_parameters_json),
                status = COALESCE(?, status),
                is_public = COALESCE(?, is_public),
                updated_at = ?,
                version = version + 1
            WHERE id = ? AND deleted = FALSE
            RETURNING *
            "#,
            request.name,
            request.description,
            request.language,
            request.gender,
            voice_type,
            request.provider_code,
            request.provider_voice_id,
            request.provider_config_json,
            request.clone_reference_url,
            request.clone_parameters_json,
            status,
            request.is_public,
            now,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn list(&self, filter: VoiceFilter, limit: i64, offset: i64) -> Result<VoiceListResult, sqlx::Error> {
        let voice_type = filter.voice_type.map(|vt| vt.as_str().to_string());
        let status = filter.status.map(|s| s.as_str().to_string());

        let voices = sqlx::query_as!(
            AudioVoice,
            r#"
            SELECT * FROM audio_voice
            WHERE deleted = FALSE
                AND (? IS NULL OR tenant_id = ?)
                AND (? IS NULL OR user_id = ?)
                AND (? IS NULL OR language = ?)
                AND (? IS NULL OR gender = ?)
                AND (? IS NULL OR voice_type = ?)
                AND (? IS NULL OR status = ?)
                AND (? IS NULL OR is_public = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            filter.tenant_id,
            filter.tenant_id,
            filter.user_id,
            filter.user_id,
            filter.language,
            filter.language,
            filter.gender,
            filter.gender,
            voice_type,
            voice_type,
            status,
            status,
            filter.is_public,
            filter.is_public,
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

        Ok(VoiceListResult {
            voices,
            total,
            has_more,
            next_cursor,
        })
    }

    async fn count(&self, filter: VoiceFilter) -> Result<i64, sqlx::Error> {
        let voice_type = filter.voice_type.map(|vt| vt.as_str().to_string());
        let status = filter.status.map(|s| s.as_str().to_string());

        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM audio_voice
            WHERE deleted = FALSE
                AND (? IS NULL OR tenant_id = ?)
                AND (? IS NULL OR user_id = ?)
                AND (? IS NULL OR language = ?)
                AND (? IS NULL OR gender = ?)
                AND (? IS NULL OR voice_type = ?)
                AND (? IS NULL OR status = ?)
                AND (? IS NULL OR is_public = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            "#,
            filter.tenant_id,
            filter.tenant_id,
            filter.user_id,
            filter.user_id,
            filter.language,
            filter.language,
            filter.gender,
            filter.gender,
            voice_type,
            voice_type,
            status,
            status,
            filter.is_public,
            filter.is_public,
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
