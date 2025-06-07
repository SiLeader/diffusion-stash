export interface Model {
  id: string;
  fileName: string;
  name: string;
  description: string;
  baseModel?: string;
  type?: string;
  createdAt: Date;
  updatedAt: Date;
}

export interface MultipleModels {
  models: Model[];
  total: number;
}
