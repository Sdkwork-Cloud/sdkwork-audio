# @sdkwork/audio-pc-generation

Speech (audio) and sound-effect (sfx) generation workspace UI for Playground and other PC surfaces.

Both modalities live in the `sdkwork-audio` domain package; SFX does not use a separate repository.

## Exports

- `@sdkwork/audio-pc-generation/react`
  - `AudioGenerationWorkspaceView` — speech / TTS (`modality="audio"`, `bucket="audios"`)
  - `SfxGenerationWorkspaceView` — sound effects (`modality="sfx"`, `bucket="sfx"`)

## Dependencies

Composition delegates to `@sdkwork/assets-pc-assets/generation-playground-workspace`.

## Verification

```bash
pnpm --filter @sdkwork/audio-pc-generation typecheck
```
