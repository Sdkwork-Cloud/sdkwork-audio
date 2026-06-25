/**
 * SDKWork Audio API Contract Tests
 *
 * Tests that verify the API contracts are valid and can generate SDKs.
 */

import { describe, it, expect } from 'vitest';
import * as fs from 'fs';
import * as path from 'path';
import * as yaml from 'js-yaml';

describe('Audio API Contracts', () => {
  const appApiPath = path.resolve(__dirname, '../../apis/app-api/audio/openapi.yaml');
  const backendApiPath = path.resolve(__dirname, '../../apis/backend-api/audio/openapi.yaml');

  describe('App API Contract', () => {
    let appApi: any;

    beforeAll(() => {
      const content = fs.readFileSync(appApiPath, 'utf8');
      appApi = yaml.load(content);
    });

    it('should have valid OpenAPI version', () => {
      expect(appApi.openapi).toBe('3.1.2');
    });

    it('should have correct info', () => {
      expect(appApi.info.title).toBe('SDKWork Audio App API');
      expect(appApi.info.version).toBe('0.1.0');
    });

    it('should have correct server URLs', () => {
      expect(appApi.servers).toHaveLength(3);
      expect(appApi.servers[0].url).toContain('/app/v3/api/audio');
    });

    it('should have security schemes', () => {
      expect(appApi.components.securitySchemes.AuthToken).toBeDefined();
      expect(appApi.components.securitySchemes.AccessToken).toBeDefined();
    });

    it('should have speech operations', () => {
      expect(appApi.paths['/speech']).toBeDefined();
      expect(appApi.paths['/speech'].post.operationId).toBe('speech.create');
      expect(appApi.paths['/speech/voices']).toBeDefined();
      expect(appApi.paths['/speech/voices'].get.operationId).toBe('speech.voices.list');
    });

    it('should have transcription operations', () => {
      expect(appApi.paths['/transcriptions']).toBeDefined();
      expect(appApi.paths['/transcriptions'].post.operationId).toBe('transcriptions.create');
      expect(appApi.paths['/transcriptions/{taskId}/segments']).toBeDefined();
    });

    it('should have translation operations', () => {
      expect(appApi.paths['/translations']).toBeDefined();
      expect(appApi.paths['/translations'].post.operationId).toBe('translations.create');
    });

    it('should have sound effect operations', () => {
      expect(appApi.paths['/sound-effects']).toBeDefined();
      expect(appApi.paths['/sound-effects'].post.operationId).toBe('soundEffects.create');
    });

    it('should have task operations', () => {
      expect(appApi.paths['/tasks']).toBeDefined();
      expect(appApi.paths['/tasks'].get.operationId).toBe('tasks.list');
      expect(appApi.paths['/tasks/{taskId}']).toBeDefined();
      expect(appApi.paths['/tasks/{taskId}'].get.operationId).toBe('tasks.retrieve');
      expect(appApi.paths['/tasks/{taskId}/cancel']).toBeDefined();
    });

    it('should have realtime operations', () => {
      expect(appApi.paths['/realtime/sessions']).toBeDefined();
      expect(appApi.paths['/realtime/sessions'].post.operationId).toBe('realtime.sessions.create');
    });

    it('should have voice operations', () => {
      expect(appApi.paths['/voices']).toBeDefined();
      expect(appApi.paths['/voices'].get.operationId).toBe('voices.list');
      expect(appApi.paths['/voices'].post.operationId).toBe('voices.create');
    });

    it('should have workspace operations', () => {
      expect(appApi.paths['/workspaces']).toBeDefined();
      expect(appApi.paths['/workspaces'].get.operationId).toBe('workspaces.list');
      expect(appApi.paths['/workspaces'].post.operationId).toBe('workspaces.create');
    });

    it('should have export operations', () => {
      expect(appApi.paths['/exports']).toBeDefined();
      expect(appApi.paths['/exports'].post.operationId).toBe('exports.create');
    });

    it('should have proper error responses', () => {
      expect(appApi.components.responses.ValidationError).toBeDefined();
      expect(appApi.components.responses.UnauthorizedError).toBeDefined();
      expect(appApi.components.responses.RateLimitError).toBeDefined();
    });

    it('should have proper schemas', () => {
      expect(appApi.components.schemas.SpeechRequest).toBeDefined();
      expect(appApi.components.schemas.TranscriptionRequest).toBeDefined();
      expect(appApi.components.schemas.TranslationRequest).toBeDefined();
      expect(appApi.components.schemas.SoundEffectRequest).toBeDefined();
      expect(appApi.components.schemas.TaskResponse).toBeDefined();
      expect(appApi.components.schemas.ErrorResponse).toBeDefined();
    });
  });

  describe('Backend API Contract', () => {
    let backendApi: any;

    beforeAll(() => {
      const content = fs.readFileSync(backendApiPath, 'utf8');
      backendApi = yaml.load(content);
    });

    it('should have valid OpenAPI version', () => {
      expect(backendApi.openapi).toBe('3.1.2');
    });

    it('should have correct info', () => {
      expect(backendApi.info.title).toBe('SDKWork Audio Backend API');
      expect(backendApi.info.version).toBe('0.1.0');
    });

    it('should have correct server URLs', () => {
      expect(backendApi.servers).toHaveLength(3);
      expect(backendApi.servers[0].url).toContain('/backend/v3/api/audio');
    });

    it('should have provider route operations', () => {
      expect(backendApi.paths['/provider-routes']).toBeDefined();
      expect(backendApi.paths['/provider-routes'].get.operationId).toBe('providerRoutes.list');
      expect(backendApi.paths['/provider-routes'].post.operationId).toBe('providerRoutes.create');
      expect(backendApi.paths['/provider-routes/{providerRouteId}']).toBeDefined();
    });

    it('should have task administration operations', () => {
      expect(backendApi.paths['/tasks']).toBeDefined();
      expect(backendApi.paths['/tasks'].get.operationId).toBe('tasks.list');
      expect(backendApi.paths['/tasks/{taskId}']).toBeDefined();
      expect(backendApi.paths['/tasks/{taskId}/retry']).toBeDefined();
      expect(backendApi.paths['/tasks/{taskId}/reconcile']).toBeDefined();
    });

    it('should have webhook operations', () => {
      expect(backendApi.paths['/provider-webhooks/{providerCode}']).toBeDefined();
      expect(backendApi.paths['/provider-webhooks/{providerCode}'].post.operationId).toBe('providerWebhooks.accept');
      expect(backendApi.paths['/webhook-events']).toBeDefined();
      expect(backendApi.paths['/webhook-events/{eventId}/replay']).toBeDefined();
    });

    it('should have analytics operations', () => {
      expect(backendApi.paths['/analytics/usage']).toBeDefined();
      expect(backendApi.paths['/analytics/usage'].get.operationId).toBe('analytics.usage');
    });
  });
});
