import { useEffect, useState } from 'react';
import { Activity, AudioWaveform, Clock3, Repeat, SlidersHorizontal, Sparkles } from 'lucide-react';
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
  updateSdkworkGenerationSfxModeConfig,
  type SdkworkGenerationAssetConfig,
  type SdkworkGenerationSfxModeConfig,
} from '../generation-asset-config';
import type { SfxGenerationPanelProps, SfxGenerationSubmitInput } from '../audio-generation-panel-types';

const SFX_RESPONSE_FORMAT_OPTIONS: NonNullable<SdkworkGenerationSfxModeConfig['responseFormat']>[] = ['mp3', 'wav'];

export function SfxGenerationPanel({
  placeholderKey,
  modelGroups,
  selectedModelId,
  onSubmitGeneration,
  submitting,
  submitError,
}: SfxGenerationPanelProps) {
  const { t } = useTranslation();
  const [prompt, setPrompt] = useState('');
  const [config, setConfig] = useState<SdkworkGenerationAssetConfig>(() =>
    createDefaultSdkworkGenerationAssetConfig('sfx'),
  );

  const selectedModel = findSdkworkGenerationModelById(modelGroups, selectedModelId)
    ?? findFirstSdkworkGenerationModelForModality(modelGroups, 'sfx');
  const normalizedPrompt = prompt.trim();
  const canSubmit = normalizedPrompt.length > 0 && !submitting && Boolean(selectedModel);
  const creditEstimate = estimateSdkworkGenerationCredits({
    config, modality: 'sfx', model: selectedModel, unavailableDetail: 'playground.generationCost.settlement',
  });

  useEffect(() => {
    setConfig((current) => reconcileSdkworkGenerationAssetConfig(current, 'sfx'));
  }, []);

  const handleSubmit = async () => {
    if (!canSubmit) return;
    const submitInput: SfxGenerationSubmitInput = {
      prompt: normalizedPrompt, selectedModality: 'sfx', targetType: 'sfx',
      selectedModel: selectedModel?.id || undefined,
      generationConfig: serializeSdkworkGenerationAssetConfig(config, 'sfx'),
    };
    await onSubmitGeneration(submitInput);
    setPrompt('');
  };

  const durationOptions = getSdkworkGenerationDurationOptions('sfx');
  const sfxMode = config.sfxMode;

  return (
    <div className="sdkwork-sfx-generation-panel flex min-h-0 flex-1 flex-col overflow-hidden">
      <div className="sdkwork-sfx-generation-panel__scroll custom-scrollbar min-h-0 flex-1 overflow-y-auto">
        <div className="sdkwork-sfx-generation-hero">
          <div className="sdkwork-sfx-generation-hero-icon" aria-hidden="true"><AudioWaveform className="h-4 w-4" /></div>
          <div className="min-w-0">
            <h3 className="sdkwork-sfx-generation-hero-title">{t('playground.sfx.studioTitle')}</h3>
            <p className="sdkwork-sfx-generation-hero-subtitle">{t('playground.sfx.studioSubtitle')}</p>
          </div>
        </div>
        <div className="sdkwork-sfx-generation-body">
          {submitError ? (<div className="sdkwork-sfx-generation-error" role="alert">{submitError}</div>) : null}
          <section className="sdkwork-sfx-generation-section">
            <div className="sdkwork-sfx-generation-section-head"><Sparkles className="h-3.5 w-3.5" aria-hidden="true" /><span>{t('playground.sfx.promptSection')}</span></div>
            <div className="sdkwork-sfx-generation-prompt-shell">
              <textarea value={prompt} onChange={(e) => setPrompt(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); void handleSubmit(); } }}
                disabled={submitting} className="sdkwork-sfx-generation-prompt-input custom-scrollbar"
                placeholder={t(placeholderKey)} rows={5} />
            </div>
            <p className="sdkwork-sfx-generation-hint">{t('playground.sfx.promptHint')}</p>
          </section>
          <section className="sdkwork-sfx-generation-section">
            <div className="sdkwork-sfx-generation-section-head"><Clock3 className="h-3.5 w-3.5" aria-hidden="true" /><span>{t('playground.sfx.durationSection')}</span></div>
            <div className="sdkwork-sfx-duration-grid">
              {durationOptions.map((duration) => (
                <button key={duration} type="button" data-active={config.durationSeconds === duration ? 'true' : 'false'}
                  onClick={() => setConfig({ ...config, durationSeconds: duration })} className="sdkwork-sfx-duration-chip">
                  <span className="font-mono text-[11px]">{duration}s</span>
                </button>
              ))}
            </div>
          </section>
          {sfxMode ? <SfxGenerationControls config={sfxMode} onChangeConfig={(next) => setConfig((c) => updateSdkworkGenerationSfxModeConfig(c, next))} /> : null}
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

export function SfxGenerationControls({ config, onChangeConfig }: {
  config: SdkworkGenerationSfxModeConfig; onChangeConfig: (config: SdkworkGenerationSfxModeConfig) => void;
}) {
  const { t } = useTranslation();
  const promptInfluence = config.promptInfluence ?? 0.3;

  return (
    <>
      <section className="sdkwork-sfx-generation-section">
        <div className="sdkwork-sfx-generation-section-head"><Activity className="h-3.5 w-3.5" aria-hidden="true" /><span>{t('playground.sfx.outputSection')}</span></div>
        <div className="sdkwork-sfx-output-grid">
          <div className="sdkwork-sfx-setting-block">
            <span className="sdkwork-sfx-setting-label">{t('playground.sfx.format')}</span>
            <div className="sdkwork-sfx-format-group" role="radiogroup" aria-label={t('playground.sfx.format')}>
              {SFX_RESPONSE_FORMAT_OPTIONS.map((format) => (
                <button key={format} type="button" role="radio" aria-checked={(config.responseFormat ?? 'mp3') === format}
                  data-active={(config.responseFormat ?? 'mp3') === format ? 'true' : 'false'}
                  onClick={() => onChangeConfig({ ...config, responseFormat: format })} className="sdkwork-sfx-format-chip">
                  {format.toUpperCase()}
                </button>
              ))}
            </div>
          </div>
          <div className="sdkwork-sfx-setting-block sdkwork-sfx-setting-block--row">
            <div className="min-w-0">
              <span className="sdkwork-sfx-setting-label">{t('playground.sfx.loop')}</span>
              <p className="sdkwork-sfx-setting-caption">{t('playground.sfx.loopDescription')}</p>
            </div>
            <button type="button" role="switch" aria-checked={config.loop} data-active={config.loop ? 'true' : 'false'}
              onClick={() => onChangeConfig({ ...config, loop: !config.loop })} className="sdkwork-sfx-toggle">
              <span className="sdkwork-sfx-toggle-thumb" />
            </button>
          </div>
        </div>
      </section>
      <section className="sdkwork-sfx-generation-section">
        <div className="sdkwork-sfx-generation-section-head"><SlidersHorizontal className="h-3.5 w-3.5" aria-hidden="true" /><span>{t('playground.sfx.fineTuneSection')}</span></div>
        <div className="sdkwork-sfx-slider-card">
          <div className="sdkwork-sfx-slider-head">
            <span className="sdkwork-sfx-setting-label">{t('playground.sfx.promptInfluence')}</span>
            <span className="sdkwork-sfx-slider-value">{Math.round(promptInfluence * 100)}%</span>
          </div>
          <input type="range" min="0" max="1" step="0.05" value={promptInfluence}
            onChange={(e) => onChangeConfig({ ...config, promptInfluence: Number(e.target.value) })}
            className="sdkwork-sfx-slider" aria-label={t('playground.sfx.promptInfluence')} />
          <div className="sdkwork-sfx-slider-scale" aria-hidden="true">
            <span>{t('playground.sfx.influenceLow')}</span><span>{t('playground.sfx.influenceHigh')}</span>
          </div>
        </div>
      </section>
    </>
  );
}
