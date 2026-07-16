/**
 * SDKWork Audio SDK Generation Contract Tests
 *
 * Tests that verify SDK generation configuration is valid.
 */

import { describe, it, expect } from 'vitest';
import * as fs from 'fs';
import * as path from 'path';

describe('SDK Generation Contracts', () => {
  const appSdkPath = path.resolve(__dirname, '../../sdks/sdkwork-audio-app-sdk');
  const backendSdkPath = path.resolve(__dirname, '../../sdks/sdkwork-audio-backend-sdk');

  describe('App SDK', () => {
    it('should have sdk-manifest.json', () => {
      const assemblyPath = path.join(appSdkPath, 'sdk-manifest.json');
      expect(fs.existsSync(assemblyPath)).toBe(true);

      const content = JSON.parse(fs.readFileSync(assemblyPath, 'utf8'));
      expect(content.workspace).toBe('sdkwork-audio-app-sdk');
      expect(content.sdkOwner).toBe('sdkwork-audio');
      expect(content.apiAuthority).toBe('sdkwork-audio-app-api');
    });

    it('should have OpenAPI spec', () => {
      const specPath = path.join(appSdkPath, 'openapi/sdkwork-audio-app-api.openapi.yaml');
      expect(fs.existsSync(specPath)).toBe(true);
    });

    it('should have component spec', () => {
      const specPath = path.join(appSdkPath, 'specs/component.spec.json');
      expect(fs.existsSync(specPath)).toBe(true);

      const content = JSON.parse(fs.readFileSync(specPath, 'utf8'));
      expect(content.component.name).toBe('@sdkwork/audio-app-sdk');
      expect(content.component.type).toBe('generated-sdk');
    });

    it('should have README', () => {
      const readmePath = path.join(appSdkPath, 'README.md');
      expect(fs.existsSync(readmePath)).toBe(true);
    });

    it('should have sdkDependencies', () => {
      const assemblyPath = path.join(appSdkPath, 'sdk-manifest.json');
      const content = JSON.parse(fs.readFileSync(assemblyPath, 'utf8'));

      expect(content.sdkDependencies).toBeDefined();
      expect(content.sdkDependencies.length).toBeGreaterThan(0);

      const driveDep = content.sdkDependencies.find((d: any) => d.workspace === 'sdkwork-drive-sdk');
      expect(driveDep).toBeDefined();
      expect(driveDep.role).toBe('audio-storage-capability');

      const iamDep = content.sdkDependencies.find((d: any) => d.workspace === 'sdkwork-iam-sdk');
      expect(iamDep).toBeDefined();
      expect(iamDep.role).toBe('authentication-capability');
    });

    it('should have multiple language targets', () => {
      const assemblyPath = path.join(appSdkPath, 'sdk-manifest.json');
      const content = JSON.parse(fs.readFileSync(assemblyPath, 'utf8'));

      expect(content.languages).toBeDefined();
      expect(content.languages.length).toBeGreaterThanOrEqual(10);

      const languages = content.languages.map((l: any) => l.language);
      expect(languages).toContain('typescript');
      expect(languages).toContain('dart');
      expect(languages).toContain('python');
      expect(languages).toContain('go');
      expect(languages).toContain('java');
      expect(languages).toContain('kotlin');
      expect(languages).toContain('swift');
      expect(languages).toContain('csharp');
      expect(languages).toContain('flutter');
      expect(languages).toContain('rust');
      expect(languages).toContain('php');
      expect(languages).toContain('ruby');
    });
  });

  describe('Backend SDK', () => {
    it('should have sdk-manifest.json', () => {
      const assemblyPath = path.join(backendSdkPath, 'sdk-manifest.json');
      expect(fs.existsSync(assemblyPath)).toBe(true);

      const content = JSON.parse(fs.readFileSync(assemblyPath, 'utf8'));
      expect(content.workspace).toBe('sdkwork-audio-backend-sdk');
      expect(content.sdkOwner).toBe('sdkwork-audio');
      expect(content.apiAuthority).toBe('sdkwork-audio-backend-api');
    });

    it('should have OpenAPI spec', () => {
      const specPath = path.join(backendSdkPath, 'openapi/sdkwork-audio-backend-api.openapi.yaml');
      expect(fs.existsSync(specPath)).toBe(true);
    });

    it('should have component spec', () => {
      const specPath = path.join(backendSdkPath, 'specs/component.spec.json');
      expect(fs.existsSync(specPath)).toBe(true);

      const content = JSON.parse(fs.readFileSync(specPath, 'utf8'));
      expect(content.component.name).toBe('@sdkwork/audio-backend-sdk');
      expect(content.component.type).toBe('generated-sdk');
    });

    it('should have README', () => {
      const readmePath = path.join(backendSdkPath, 'README.md');
      expect(fs.existsSync(readmePath)).toBe(true);
    });
  });
});
