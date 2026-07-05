import { useEffect, useState } from 'react';
import { AlertCircle, Gauge, Mic2, Volume2 } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { SdkworkStudioGenerationBottomBar } from '@sdkwork/generations-pc-studio/react';
import {
  createDefaultSdkworkGenerationAssetConfig,
  estimateSdkworkGenerationCredits,
  findFirstSdkworkGenerationModelForModality,
  findSdkworkGenerationModelById,
  getSdkworkGenerationDurationOptions,
  reconcileSdkworkGenerationAssetConfig,
  serializeSdkworkGenerationAssetConfig,
  updateSdkworkGenerationSpeechModeConfig,
  type SdkworkGenerationAssetConfig,
  type SdkworkGenerationSpeechModeConfig,
} from '../generation-asset-config';
import type { AudioGenerationPanelProps, AudioGenerationSubmitInput } from '../audio-generation-panel-types';

const SPEECH_VOICE_OPTIONS = ['alloy', 'ash', 'ballad', 'coral', 'echo', 'fable', 'onyx', 'nova', 'sage', 'shimmer', 'Kore', 'Puck', 'Charon', 'Fenrir', 'Aoede'] as const;
const SPEECH_RESPONSE_FORMAT_OPTIONS: NonNullable<SdkworkGenerationSpeechModeConfig['responseFormat']>[] = ['mp3', 'wav', 'aac', 'flac', 'opus', 'pcm'];

export function AudioGenerationPanel({
  placeholderKey,
  modelGroups,
  selectedModelId,
  onSubmitGeneration,
  submitting,
  submitError,
}: AudioGenerationPanelProps) {
  const { t } = useTranslation();
  const [prompt, setPrompt] = useState('');
  const [config, setConfig] = useState<SdkworkGenerationAssetConfig>(() =>
    createDefaultSdkworkGenerationAssetConfig('audio'),
  );

  const selectedModel = findSdkworkGenerationModelById(modelGroups, selectedModelId)
    ?? findFirstSdkworkGenerationModelForModality(modelGroups, 'audio');
  const normalizedPrompt = prompt.trim();
  const canSubmit = normalizedPrompt.length > 0 && !submitting && Boolean(selectedModel);
  const creditEstimate = estimateSdkworkGenerationCredits({
    config, modality: 'audio', model: selectedModel, unavailableDetail: 'playground.generationCost.settlement',
  });

  useEffect(() => {
    setConfig((current) => reconcileSdkworkGenerationAssetConfig(current, 'audio'));
  }, []);

  const handleSubmit = async () => {
    if (!canSubmit) return;
    const submitInput: AudioGenerationSubmitInput = {
      prompt: normalizedPrompt, selectedModality: 'audio', targetType: 'audio',
      selectedModel: selectedModel?.id || undefined,
      generationConfig: serializeSdkworkGenerationAssetConfig(config, 'audio'),
    };
    await onSubmitGeneration(submitInput);
    setPrompt('');
  };

  const durationOptions = getSdkworkGenerationDurationOptions('audio');

  return (
    <div className="sdkwork-studio-panel flex min-h-0 flex-1 flex-col overflow-hidden">
      <div className="sdkwork-studio-hero">
        <div className="sdkwork-studio-hero-icon" aria-hidden="true">
          <Mic2 className="h-4 w-4" />
        </div>
        <div className="min-w-0">
          <div className="sdkwork-studio-hero-title">{t('playground.audio.studioTitle')}</div>
          <div className="sdkwork-studio-hero-subtitle">{t('playground.audio.studioSubtitle')}</div>
        </div>
      </div>

      <div className="sdkwork-studio-scroll custom-scrollbar">
        <div className="sdkwork-studio-body">
          {submitError ? (
            <div className="sdkwork-studio-error" role="alert">
              <AlertCircle className="mt-0.5 h-4 w-4 shrink-0 text-red-400" />
              <span className="leading-relaxed">{submitError}</span>
            </div>
          ) : null}

          {config.speechMode ? (
            <section className="sdkwork-studio-section">
              <div className="sdkwork-studio-section-head">
                <span className="sdkwork-studio-section-head__label">
                  <Mic2 className="h-3.5 w-3.5" aria-hidden="true" />
                  {t('playground.audio.voiceSection')}
                </span>
              </div>
              <div className="sdkwork-studio-controls">
                <div className="sdkwork-studio-controls__grid">
                  <label className="sdkwork-studio-field">
                    <span className="sdkwork-studio-field__label">
                      <Mic2 className="h-3.5 w-3.5" aria-hidden="true" />
                      {t('playground.speech.voice')}
                    </span>
                    <select
                      value={config.speechMode.voice ?? ''}
                      onChange={(e) => setConfig((c) => updateSdkworkGenerationSpeechModeConfig(c, { ...c.speechMode!, voice: e.target.value || undefined }))}
                      className="sdkwork-studio-select"
                    >
                      <option value="">{t('playground.speech.voiceAuto')}</option>
                      {SPEECH_VOICE_OPTIONS.map((voice) => <option key={voice} value={voice}>{voice}</option>)}
                    </select>
                  </label>
                  <label className="sdkwork-studio-field">
                    <span className="sdkwork-studio-field__label">
                      <Volume2 className="h-3.5 w-3.5" aria-hidden="true" />
                      {t('playground.speech.format')}
                    </span>
                    <select
                      value={config.speechMode.responseFormat ?? 'mp3'}
                      onChange={(e) => setConfig((c) => updateSdkworkGenerationSpeechModeConfig(c, { ...c.speechMode!, responseFormat: e.target.value as SdkworkGenerationSpeechModeConfig['responseFormat'] }))}
                      className="sdkwork-studio-select"
                    >
                      {SPEECH_RESPONSE_FORMAT_OPTIONS.map((format) => <option key={format} value={format}>{format.toUpperCase()}</option>)}
                    </select>
                  </label>
                </div>
                <label className="sdkwork-studio-field">
                  <span className="sdkwork-studio-field__label sdkwork-studio-section-head">
                    <Gauge className="h-3.5 w-3.5" aria-hidden="true" />
                    {t('playground.speech.speed')}
                    <span className="ml-auto font-mono normal-case">{(config.speechMode.speed ?? 1).toFixed(2)}x</span>
                  </span>
                  <input
                    type="range"
                    min="0.25"
                    max="4"
                    step="0.05"
                    value={config.speechMode.speed ?? 1}
                    onChange={(e) => setConfig((c) => updateSdkworkGenerationSpeechModeConfig(c, { ...c.speechMode!, speed: Number(e.target.value) }))}
                    className="sdkwork-studio-slider"
                  />
                </label>
              </div>
            </section>
          ) : null}

          <div className="sdkwork-studio-prompt">
            <div className="sdkwork-studio-prompt__header">
              <span>{t('playground.audio.promptSection')}</span>
              <span className="hidden max-w-[46%] truncate text-[10px] normal-case sm:inline">
                {t('playground.audio.promptHint')}
              </span>
            </div>
            <textarea
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); void handleSubmit(); } }}
              className="sdkwork-studio-prompt__textarea custom-scrollbar"
              placeholder={t(placeholderKey)}
            />
            <div className="sdkwork-studio-prompt__footer">
              <span>
                <kbd className="sdkwork-studio-prompt__kbd">Enter</kbd>
                <span className="ml-1.5">{t('playground.promptKeyboard.submit')}</span>
              </span>
              <span className="tabular-nums">{normalizedPrompt.length}</span>
            </div>
          </div>

          <section className="sdkwork-studio-section">
            <div className="sdkwork-studio-section-head">
              <span>{t('playground.audio.durationSection')}</span>
              <span className="font-mono normal-case">{config.durationSeconds}s</span>
            </div>
            <div className="sdkwork-studio-chip-grid">
              {durationOptions.map((duration) => (
                <button
                  key={duration}
                  type="button"
                  data-active={config.durationSeconds === duration ? 'true' : 'false'}
                  onClick={() => setConfig({ ...config, durationSeconds: duration })}
                  className="sdkwork-studio-chip"
                >
                  <Volume2 className="h-3.5 w-3.5" aria-hidden="true" />
                  {duration}s
                </button>
              ))}
            </div>
          </section>
        </div>
      </div>

      <SdkworkStudioGenerationBottomBar
        canSubmit={canSubmit}
        creditEstimate={creditEstimate}
        onSubmit={handleSubmit}
        submitting={submitting}
      />
    </div>
  );
}
