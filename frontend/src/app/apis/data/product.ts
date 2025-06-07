import {Model} from './model';

export interface Product {
  id: string;
  name: string;
  models: Model[];
  mimeType: string;
  positivePrompt?: string;
  negativePrompt?: string;
  samplerName?: string;
  schedulerName?: string;
  stepCount?: number;
  cfgScale?: number;
  seed?: number;
  createdAt: Date;
  updatedAt: Date;
}

export interface MultipleProducts {
  products: Product[];
  total: number;
}

export interface GenerationInfo {
  modelNames: string[];
  positivePrompt?: string;
  negativePrompt?: string;
  stepCount?: number;
  cfgScale?: number;
  seed?: number;
  samplerName?: string;
  schedulerName?: string;
}

export interface DecodedPng {
  resolvedModels: Model[];
  decoded: GenerationInfo,
}
