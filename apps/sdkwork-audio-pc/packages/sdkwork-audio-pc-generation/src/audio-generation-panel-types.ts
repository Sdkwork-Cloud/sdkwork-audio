import type { SdkworkGenerationSerializedAssetConfig } from './generation-asset-config';

export interface AudioGenerationModelOption {
  id: string;
  name?: string;
  displayName?: string;
  vendorCode?: string;
  vendorName?: string;
}

export interface AudioGenerationModelGroup {
  id: string;
  llms: AudioGenerationModelOption[];
  audios: AudioGenerationModelOption[];
  sfx: AudioGenerationModelOption[];
}

export interface AudioGenerationSubmitInput {
  prompt: string;
  selectedModality: 'audio';
  targetType?: 'audio';
  selectedModel?: string;
  generationConfig?: SdkworkGenerationSerializedAssetConfig;
}

export interface SfxGenerationSubmitInput {
  prompt: string;
  selectedModality: 'sfx';
  targetType?: 'sfx';
  selectedModel?: string;
  generationConfig?: SdkworkGenerationSerializedAssetConfig;
}

export interface AudioGenerationPanelProps {
  placeholderKey: string;
  modelGroups: AudioGenerationModelGroup[];
  selectedModelId: string;
  onSubmitGeneration: (input: AudioGenerationSubmitInput) => Promise<void>;
  submitting: boolean;
  submitError: string | null;
}

export interface SfxGenerationPanelProps {
  placeholderKey: string;
  modelGroups: AudioGenerationModelGroup[];
  selectedModelId: string;
  onSubmitGeneration: (input: SfxGenerationSubmitInput) => Promise<void>;
  submitting: boolean;
  submitError: string | null;
}
