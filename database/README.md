# AUDIO Database Module

Canonical lifecycle assets for `sdkwork-audio` per `DATABASE_FRAMEWORK_SPEC.md`.

## Commands

```bash
pnpm run db:materialize:contract
pnpm run db:validate
```

Legacy SQL: `crates/sdkwork-audio-generation-repository-sqlx/migrations/0001_audio_core.sql` → `database/ddl/baseline/postgres/0001_audio_legacy_baseline.sql`

Runtime bootstrap: `sdkwork-audio-database-host` / `connect_and_bootstrap_audio_database_from_env()`.
