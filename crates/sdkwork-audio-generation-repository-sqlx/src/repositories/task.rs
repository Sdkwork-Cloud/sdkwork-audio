//! Task repository

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::entities::task::*;

/// Task repository trait
#[async_trait]
pub trait TaskRepository {
    async fn create(&self, request: CreateTaskRequest) -> Result<AudioGenerationTask, sqlx::Error>;
    async fn get_by_id(&self, id: i64) -> Result<Option<AudioGenerationTask>, sqlx::Error>;
    async fn get_by_task_no(&self, task_no: &str) -> Result<Option<AudioGenerationTask>, sqlx::Error>;
    async fn get_by_idempotency_key(
        &self,
        tenant_id: i64,
        operation_type: &str,
        idempotency_key: &str,
    ) -> Result<Option<AudioGenerationTask>, sqlx::Error>;
    async fn update(&self, id: i64, request: UpdateTaskRequest) -> Result<AudioGenerationTask, sqlx::Error>;
    async fn list(&self, filter: TaskFilter, limit: i64, offset: i64) -> Result<TaskListResult, sqlx::Error>;
    async fn count(&self, filter: TaskFilter) -> Result<i64, sqlx::Error>;
}

/// SQLx implementation of TaskRepository
pub struct SqliteTaskRepository {
    pool: SqlitePool,
}

impl SqliteTaskRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for SqliteTaskRepository {
    async fn create(&self, request: CreateTaskRequest) -> Result<AudioGenerationTask, sqlx::Error> {
        let task_no = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let status = TaskStatus::Queued.as_str();
        let operation_type = request.operation_type.as_str();

        let result = sqlx::query_as!(
            AudioGenerationTask,
            r#"
            INSERT INTO audio_generation_task (
                task_no, tenant_id, organization_id, user_id, operation_type,
                provider_code, provider_route_id, model, idempotency_key,
                status, progress, request_json, callback_url,
                created_at, updated_at, deleted, version
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#,
            task_no,
            request.tenant_id,
            request.organization_id,
            request.user_id,
            operation_type,
            request.provider_code,
            request.provider_route_id,
            request.model,
            request.idempotency_key,
            status,
            0,
            request.request_json,
            request.callback_url,
            now,
            now,
            false,
            0
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<AudioGenerationTask>, sqlx::Error> {
        let task = sqlx::query_as!(
            AudioGenerationTask,
            r#"
            SELECT * FROM audio_generation_task
            WHERE id = ? AND deleted = FALSE
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn get_by_task_no(&self, task_no: &str) -> Result<Option<AudioGenerationTask>, sqlx::Error> {
        let task = sqlx::query_as!(
            AudioGenerationTask,
            r#"
            SELECT * FROM audio_generation_task
            WHERE task_no = ? AND deleted = FALSE
            "#,
            task_no
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn get_by_idempotency_key(
        &self,
        tenant_id: i64,
        operation_type: &str,
        idempotency_key: &str,
    ) -> Result<Option<AudioGenerationTask>, sqlx::Error> {
        let task = sqlx::query_as!(
            AudioGenerationTask,
            r#"
            SELECT * FROM audio_generation_task
            WHERE tenant_id = ? AND operation_type = ? AND idempotency_key = ? AND deleted = FALSE
            "#,
            tenant_id,
            operation_type,
            idempotency_key
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(task)
    }

    async fn update(&self, id: i64, request: UpdateTaskRequest) -> Result<AudioGenerationTask, sqlx::Error> {
        let now = chrono::Utc::now();
        let status = request.status.map(|s| s.as_str().to_string());

        let result = sqlx::query_as!(
            AudioGenerationTask,
            r#"
            UPDATE audio_generation_task SET
                provider_code = COALESCE(?, provider_code),
                provider_route_id = COALESCE(?, provider_route_id),
                provider_task_id = COALESCE(?, provider_task_id),
                status = COALESCE(?, status),
                progress = COALESCE(?, progress),
                provider_request_json = COALESCE(?, provider_request_json),
                provider_response_json = COALESCE(?, provider_response_json),
                result_json = COALESCE(?, result_json),
                error_code = COALESCE(?, error_code),
                error_message = COALESCE(?, error_message),
                callback_status = COALESCE(?, callback_status),
                submitted_at = COALESCE(?, submitted_at),
                completed_at = COALESCE(?, completed_at),
                expires_at = COALESCE(?, expires_at),
                updated_at = ?,
                version = version + 1
            WHERE id = ? AND deleted = FALSE
            RETURNING *
            "#,
            request.provider_code,
            request.provider_route_id,
            request.provider_task_id,
            status,
            request.progress,
            request.provider_request_json,
            request.provider_response_json,
            request.result_json,
            request.error_code,
            request.error_message,
            request.callback_status,
            request.submitted_at,
            request.completed_at,
            request.expires_at,
            now,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn list(&self, filter: TaskFilter, limit: i64, offset: i64) -> Result<TaskListResult, sqlx::Error> {
        let operation_type = filter.operation_type.map(|o| o.as_str().to_string());
        let status = filter.status.map(|s| s.as_str().to_string());

        let tasks = sqlx::query_as!(
            AudioGenerationTask,
            r#"
            SELECT * FROM audio_generation_task
            WHERE deleted = FALSE
                AND (? IS NULL OR tenant_id = ?)
                AND (? IS NULL OR user_id = ?)
                AND (? IS NULL OR organization_id = ?)
                AND (? IS NULL OR operation_type = ?)
                AND (? IS NULL OR status = ?)
                AND (? IS NULL OR provider_code = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            filter.tenant_id,
            filter.tenant_id,
            filter.user_id,
            filter.user_id,
            filter.organization_id,
            filter.organization_id,
            operation_type,
            operation_type,
            status,
            status,
            filter.provider_code,
            filter.provider_code,
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

        Ok(TaskListResult {
            tasks,
            total,
            has_more,
            next_cursor,
        })
    }

    async fn count(&self, filter: TaskFilter) -> Result<i64, sqlx::Error> {
        let operation_type = filter.operation_type.map(|o| o.as_str().to_string());
        let status = filter.status.map(|s| s.as_str().to_string());

        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM audio_generation_task
            WHERE deleted = FALSE
                AND (? IS NULL OR tenant_id = ?)
                AND (? IS NULL OR user_id = ?)
                AND (? IS NULL OR organization_id = ?)
                AND (? IS NULL OR operation_type = ?)
                AND (? IS NULL OR status = ?)
                AND (? IS NULL OR provider_code = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            "#,
            filter.tenant_id,
            filter.tenant_id,
            filter.user_id,
            filter.user_id,
            filter.organization_id,
            filter.organization_id,
            operation_type,
            operation_type,
            status,
            status,
            filter.provider_code,
            filter.provider_code,
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
