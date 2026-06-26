/**
 * SDKWork Audio Rust Crates Contract Tests
 *
 * Tests that verify Rust crate structure is valid.
 */

import { describe, it, expect } from 'vitest';
import * as fs from 'fs';
import * as path from 'path';

describe('Rust Crates Contracts', () => {
  const cratesPath = path.resolve(__dirname, '../../crates');

  const expectedCrates = [
    'sdkwork-routes-audio-app-api',
    'sdkwork-routes-audio-backend-api',
    'sdkwork-audio-artifact-drive-service',
    'sdkwork-audio-generation-repository-sqlx',
    'sdkwork-audio-realtime-rust',
    'sdkwork-audio-ai-engine-rust',
  ];

  describe('Workspace Cargo.toml', () => {
    let cargoContent: string;

    beforeAll(() => {
      const cargoPath = path.resolve(__dirname, '../../Cargo.toml');
      expect(fs.existsSync(cargoPath)).toBe(true);
      cargoContent = fs.readFileSync(cargoPath, 'utf8');
    });

    it('should have workspace resolver', () => {
      expect(cargoContent).toContain('resolver = "2"');
    });

    it('should have all crate members', () => {
      for (const crate of expectedCrates) {
        expect(cargoContent).toContain(`crates/${crate}`);
      }
    });

    it('should have workspace dependencies', () => {
      expect(cargoContent).toContain('[workspace.dependencies]');
      expect(cargoContent).toContain('serde = { version = "1", features = ["derive"] }');
      expect(cargoContent).toContain('serde_json = "1"');
      expect(cargoContent).toContain('tokio = { version = "1.48"');
      expect(cargoContent).toContain('uuid = { version = "1", features = ["v4"] }');
    });
  });

  for (const crateName of expectedCrates) {
    describe(`Crate: ${crateName}`, () => {
      const cratePath = path.join(cratesPath, crateName);

      it('should exist', () => {
        expect(fs.existsSync(cratePath)).toBe(true);
      });

      it('should have Cargo.toml', () => {
        const cargoPath = path.join(cratePath, 'Cargo.toml');
        expect(fs.existsSync(cargoPath)).toBe(true);

        const content = fs.readFileSync(cargoPath, 'utf8');
        expect(content).toContain(`name = "${crateName}"`);
        expect(content).toContain('version = "0.1.0"');
        expect(content).toContain('edition = "2021"');
      });

      it('should have src directory', () => {
        const srcPath = path.join(cratePath, 'src');
        expect(fs.existsSync(srcPath)).toBe(true);
      });

      it('should have lib.rs', () => {
        const libPath = path.join(cratePath, 'src/lib.rs');
        expect(fs.existsSync(libPath)).toBe(true);

        const content = fs.readFileSync(libPath, 'utf8');
        expect(content.length).toBeGreaterThan(0);
      });
    });
  }

  describe('Naming Compliance', () => {
    it('should have route crates with correct naming', () => {
      const routeCrates = expectedCrates.filter((c) => c.startsWith('sdkwork-routes-'));
      expect(routeCrates.length).toBe(2);
      expect(routeCrates).toContain('sdkwork-routes-audio-app-api');
      expect(routeCrates).toContain('sdkwork-routes-audio-backend-api');
    });

    it('should have repository crate with correct naming', () => {
      const repoCrates = expectedCrates.filter((c) => c.endsWith('-repository-sqlx'));
      expect(repoCrates.length).toBe(1);
      expect(repoCrates).toContain('sdkwork-audio-generation-repository-sqlx');
    });

    it('should have service crate with correct naming', () => {
      const serviceCrates = expectedCrates.filter((c) => c.endsWith('-service'));
      expect(serviceCrates.length).toBe(1);
      expect(serviceCrates).toContain('sdkwork-audio-artifact-drive-service');
    });
  });
});
