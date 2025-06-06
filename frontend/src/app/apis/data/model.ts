export interface Model {
  id: string;
  fileName: string;
  name: string;
  description: string;
  category?: string;
  modelType?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface MultipleModels {
  models: Model[];
  total: number;
}
