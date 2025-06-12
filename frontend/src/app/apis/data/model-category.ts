export interface BaseModel {
  name: string;
  id: string;
}

export interface ModelType {
  name: string;
  id: string;
}

export interface SelectOption {
  value: string | null;
  label: string;
}
