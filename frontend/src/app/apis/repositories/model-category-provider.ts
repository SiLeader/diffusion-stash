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
        {id: 'StableDiffusion15', name: 'Stable Diffusion 1.5'},
        {id: 'StableDiffusionXl', name: 'Stable Diffusion XL'},
        {id: 'PonyDiffusion', name: 'Pony Diffusion'},
        {id: 'Illustrious', name: 'Illustrious'},
        {id: 'StableDiffusion30', name: 'Stable Diffusion 3'},
        {id: 'StableDiffusion35', name: 'Stable Diffusion 3.5'},
        {id: 'Flux1D', name: 'Flux.1 D'},
        {id: 'Flux1S', name: 'Flux.1 S'},
      ]
    });
  }

  fetchModelTypes(): Resource<ModelType[]> {
    return resource({
      defaultValue: [],
      loader: async () => [
        {id: 'Checkpoint', name: 'Checkpoint'},
        {id: 'Lora', name: 'LoRA'},
        {id: 'Lycoris', name: 'LyCORIS'},
        {id: 'Vae', name: 'VAE'},
        {id: 'Embedding', name: 'Embedding'},
        {id: 'Upscaler', name: 'Upscaler Model'},
        {id: 'ControlNet', name: 'ControlNet Model'},
      ]
    });
  }

  fetchSelectableModelTypes(modelTypesResource: Resource<ModelType[]>): Signal<SelectOption[]> {
    return computed(() => {
      const types = modelTypesResource.value()?.map(type => ({
        value: type.id,
        label: type.name
      })) || [];
      return [...types, {value: null, label: 'Other'}];
    });
  }

  fetchSelectableBaseModels(baseModelsResource: Resource<BaseModel[]>): Signal<SelectOption[]> {
    return computed(() => {
      const baseModels = baseModelsResource.value()?.map(model => ({
        value: model.id,
        label: model.name
      })) || [];
      return [...baseModels, {value: null, label: 'Other'}];
    });

  }
}
