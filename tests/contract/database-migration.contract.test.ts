/**
 * SDKWork Audio Database Migration Contract Tests
 *
 * Tests that verify database migrations are valid and complete.
 */

import { describe, it, expect } from 'vitest';
import * as fs from 'fs';
import * as path from 'path';

describe('Database Migration Contracts', () => {
  const migrationsPath = path.resolve(
    __dirname,
    '../../crates/sdkwork-audio-generation-repository-sqlx/migrations'
  );

  it('should have migrations directory', () => {
    expect(fs.existsSync(migrationsPath)).toBe(true);
  });

  it('should have at least one migration file', () => {
    const files = fs.readdirSync(migrationsPath);
    const sqlFiles = files.filter((f) => f.endsWith('.sql'));
    expect(sqlFiles.length).toBeGreaterThan(0);
  });

  describe('Migration 0001_audio_core.sql', () => {
    let migrationContent: string;

    beforeAll(() => {
      const migrationPath = path.join(migrationsPath, '0001_audio_core.sql');
      expect(fs.existsSync(migrationPath)).toBe(true);
      migrationContent = fs.readFileSync(migrationPath, 'utf8');
    });

    it('should create audio_provider_route table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_provider_route');
      expect(migrationContent).toContain('id BIGINT PRIMARY KEY');
      expect(migrationContent).toContain('route_key VARCHAR(128) NOT NULL');
      expect(migrationContent).toContain('provider_id VARCHAR(64) NOT NULL');
    });

    it('should create audio_generation_task table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_generation_task');
      expect(migrationContent).toContain('task_no VARCHAR(64) NOT NULL');
      expect(migrationContent).toContain('tenant_id BIGINT NOT NULL');
      expect(migrationContent).toContain('user_id BIGINT NOT NULL');
      expect(migrationContent).toContain('operation_type VARCHAR(32) NOT NULL');
      expect(migrationContent).toContain('status VARCHAR(32) NOT NULL');
    });

    it('should create audio_task_event table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_task_event');
      expect(migrationContent).toContain('event_no VARCHAR(64) NOT NULL');
      expect(migrationContent).toContain('task_id BIGINT NOT NULL');
      expect(migrationContent).toContain('event_type VARCHAR(64) NOT NULL');
    });

    it('should create audio_audio_artifact table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_audio_artifact');
      expect(migrationContent).toContain('artifact_no VARCHAR(64) NOT NULL');
      expect(migrationContent).toContain('kind VARCHAR(32) NOT NULL');
    });

    it('should create audio_artifact_drive_sync table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_artifact_drive_sync');
      expect(migrationContent).toContain('sync_no VARCHAR(64) NOT NULL');
      expect(migrationContent).toContain('sync_status VARCHAR(32) NOT NULL');
    });

    it('should create audio_voice table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_voice');
      expect(migrationContent).toContain('voice_no VARCHAR(64) NOT NULL');
      expect(migrationContent).toContain('name VARCHAR(255) NOT NULL');
      expect(migrationContent).toContain('language VARCHAR(10) NOT NULL');
    });

    it('should create audio_realtime_session table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_realtime_session');
      expect(migrationContent).toContain('session_no VARCHAR(64) NOT NULL');
      expect(migrationContent).toContain('session_type VARCHAR(50) NOT NULL');
    });

    it('should create audio_workspace table', () => {
      expect(migrationContent).toContain('CREATE TABLE IF NOT EXISTS audio_workspace');
      expect(migrationContent).toContain('workspace_no VARCHAR(64) NOT NULL');
      expect(migrationContent).toContain('name VARCHAR(255) NOT NULL');
    });

    it('should have proper indexes', () => {
      expect(migrationContent).toContain('CREATE INDEX idx_audio_generation_task_tenant_user');
      expect(migrationContent).toContain('CREATE INDEX idx_audio_generation_task_status');
      expect(migrationContent).toContain('CREATE INDEX idx_audio_generation_task_operation');
      expect(migrationContent).toContain('CREATE INDEX idx_audio_voice_tenant');
      expect(migrationContent).toContain('CREATE INDEX idx_audio_realtime_session_user');
    });

    it('should have unique constraints', () => {
      expect(migrationContent).toContain('CONSTRAINT uk_audio_provider_route_key UNIQUE');
      expect(migrationContent).toContain('CONSTRAINT uk_audio_generation_task_no UNIQUE');
      expect(migrationContent).toContain('CONSTRAINT uk_audio_task_idempotency UNIQUE');
      expect(migrationContent).toContain('CONSTRAINT uk_audio_voice_no UNIQUE');
    });

    it('should have audit fields', () => {
      expect(migrationContent).toContain('created_at TIMESTAMP NOT NULL');
      expect(migrationContent).toContain('updated_at TIMESTAMP NOT NULL');
      expect(migrationContent).toContain('deleted BOOLEAN NOT NULL DEFAULT FALSE');
      expect(migrationContent).toContain('version BIGINT NOT NULL DEFAULT 0');
    });

    it('should have multi-tenant support', () => {
      expect(migrationContent).toContain('tenant_id BIGINT NOT NULL DEFAULT 0');
      expect(migrationContent).toContain('organization_id BIGINT NOT NULL DEFAULT 0');
      expect(migrationContent).toContain('user_id BIGINT NOT NULL DEFAULT 0');
    });
  });
});
