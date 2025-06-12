import {computed, Injectable, resource, Resource, Signal} from '@angular/core';
import {BaseModel, ModelType, SelectOption} from '../data/model-category';

@Injectable({
  providedIn: 'root'
})
export class ModelCategoryProvider {

  constructor() {
  }

  fetchBaseModels(): Resource<BaseModel[]> {
    return resource({
      defaultValue: [],
      loader: async () => [
        {key: 'StableDiffusion15', name: 'Stable Diffusion 1.5'},
        {key: 'StableDiffusionXl', name: 'Stable Diffusion XL'},
        {key: 'PonyDiffusion', name: 'Pony Diffusion'},
        {key: 'Illustrious', name: 'Illustrious'},
        {key: 'StableDiffusion30', name: 'Stable Diffusion 3'},
        {key: 'StableDiffusion35', name: 'Stable Diffusion 3.5'},
        {key: 'Flux1D', name: 'Flux.1 D'},
        {key: 'Flux1S', name: 'Flux.1 S'},
      ]
    });
  }

  fetchModelTypes(): Resource<ModelType[]> {
    return resource({
      defaultValue: [],
      loader: async () => [
        {key: 'Checkpoint', name: 'Checkpoint'},
        {key: 'Lora', name: 'LoRA'},
        {key: 'Lycoris', name: 'LyCORIS'},
        {key: 'Vae', name: 'VAE'},
        {key: 'Embedding', name: 'Embedding'},
        {key: 'Upscaler', name: 'Upscaler Model'},
        {key: 'ControlNet', name: 'ControlNet Model'},
      ]
    });
  }

  fetchSelectableModelTypes(): Signal<SelectOption[]> {
    return computed(() => {
      const types = this.fetchModelTypes().value()?.map(type => ({
        value: type.id,
        label: type.name
      })) || [];
      return [...types, {value: null, label: 'Other'}];
    });
  }

  fetchSelectableBaseModels(): Signal<SelectOption[]> {
    return computed(() => {
      const baseModels = this.fetchBaseModels().value()?.map(model => ({
        value: model.id,
        label: model.name
      })) || [];
      return [...baseModels, {value: null, label: 'Other'}];
    });

  }
}
