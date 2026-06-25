//! Event repository

use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::entities::event::*;

/// Event repository trait
#[async_trait]
pub trait EventRepository {
    async fn create(&self, request: CreateEventRequest) -> Result<AudioTaskEvent, sqlx::Error>;
    async fn get_by_id(&self, id: i64) -> Result<Option<AudioTaskEvent>, sqlx::Error>;
    async fn get_by_event_no(&self, event_no: &str) -> Result<Option<AudioTaskEvent>, sqlx::Error>;
    async fn list_by_task(&self, task_id: i64, limit: i64, offset: i64) -> Result<EventListResult, sqlx::Error>;
    async fn list(&self, filter: EventFilter, limit: i64, offset: i64) -> Result<EventListResult, sqlx::Error>;
    async fn count(&self, filter: EventFilter) -> Result<i64, sqlx::Error>;
}

/// SQLx implementation of EventRepository
pub struct SqliteEventRepository {
    pool: SqlitePool,
}

impl SqliteEventRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventRepository for SqliteEventRepository {
    async fn create(&self, request: CreateEventRequest) -> Result<AudioTaskEvent, sqlx::Error> {
        let event_no = Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let event_type = request.event_type.as_str();
        let status = "processed";

        let result = sqlx::query_as!(
            AudioTaskEvent,
            r#"
            INSERT INTO audio_task_event (
                event_no, task_id, event_type, from_status, to_status,
                provider_code, provider_event_id, provider_task_id,
                payload_json, received_at, processed_at,
                status, message, created_at, updated_at, deleted, version
            ) VALUES (
                ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?
            )
            RETURNING *
            "#,
            event_no,
            request.task_id,
            event_type,
            request.from_status,
            request.to_status,
            request.provider_code,
            request.provider_event_id,
            request.provider_task_id,
            request.payload_json,
            now,
            now,
            status,
            request.message,
            now,
            now,
            false,
            0
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<AudioTaskEvent>, sqlx::Error> {
        let event = sqlx::query_as!(
            AudioTaskEvent,
            r#"
            SELECT * FROM audio_task_event
            WHERE id = ? AND deleted = FALSE
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(event)
    }

    async fn get_by_event_no(&self, event_no: &str) -> Result<Option<AudioTaskEvent>, sqlx::Error> {
        let event = sqlx::query_as!(
            AudioTaskEvent,
            r#"
            SELECT * FROM audio_task_event
            WHERE event_no = ? AND deleted = FALSE
            "#,
            event_no
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(event)
    }

    async fn list_by_task(&self, task_id: i64, limit: i64, offset: i64) -> Result<EventListResult, sqlx::Error> {
        let events = sqlx::query_as!(
            AudioTaskEvent,
            r#"
            SELECT * FROM audio_task_event
            WHERE task_id = ? AND deleted = FALSE
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            task_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM audio_task_event
            WHERE task_id = ? AND deleted = FALSE
            "#,
            task_id
        )
        .fetch_one(&self.pool)
        .await?;

        let total = total.unwrap_or(0);
        let has_more = (offset + limit) < total;
        let next_cursor = if has_more {
            Some((offset + limit).to_string())
        } else {
            None
        };

        Ok(EventListResult {
            events,
            total,
            has_more,
            next_cursor,
        })
    }

    async fn list(&self, filter: EventFilter, limit: i64, offset: i64) -> Result<EventListResult, sqlx::Error> {
        let event_type = filter.event_type.map(|et| et.as_str().to_string());

        let events = sqlx::query_as!(
            AudioTaskEvent,
            r#"
            SELECT * FROM audio_task_event
            WHERE deleted = FALSE
                AND (? IS NULL OR task_id = ?)
                AND (? IS NULL OR event_type = ?)
                AND (? IS NULL OR provider_code = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            filter.task_id,
            filter.task_id,
            event_type,
            event_type,
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

        Ok(EventListResult {
            events,
            total,
            has_more,
            next_cursor,
        })
    }

    async fn count(&self, filter: EventFilter) -> Result<i64, sqlx::Error> {
        let event_type = filter.event_type.map(|et| et.as_str().to_string());

        let result = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as count FROM audio_task_event
            WHERE deleted = FALSE
                AND (? IS NULL OR task_id = ?)
                AND (? IS NULL OR event_type = ?)
                AND (? IS NULL OR provider_code = ?)
                AND (? IS NULL OR created_at >= ?)
                AND (? IS NULL OR created_at <= ?)
            "#,
            filter.task_id,
            filter.task_id,
            event_type,
            event_type,
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
