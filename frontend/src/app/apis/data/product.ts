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
