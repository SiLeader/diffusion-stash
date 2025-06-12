import {Component, computed, input, output, Resource, signal, Signal} from '@angular/core';
import {MatFormField, MatOption, MatSelect} from '@angular/material/select';
import {ModelCategoryProvider} from '../../../apis/repositories/model-category-provider';
import {BaseModel, ModelType, SelectOption} from '../../../apis/data/model-category';
import {Model, MultipleModels} from '../../../apis/data/model';
import {FetchListOptions, ModelRepository} from '../../../apis/repositories/model-repository';
import {FormsModule} from '@angular/forms';

@Component({
  selector: 'app-model-selector',
  imports: [
    MatSelect,
    MatFormField,
    MatOption,
    FormsModule
  ],
  templateUrl: './model-selector.html',
  styleUrl: './model-selector.css'
})
export class ModelSelector {
  constructor(
    modelCategoryProvider: ModelCategoryProvider,
    modelRepository: ModelRepository,
  ) {
    this.selectableTypes = modelCategoryProvider.fetchSelectableModelTypes();
    this.selectableBaseModels = modelCategoryProvider.fetchSelectableBaseModels();

    const options = computed<Partial<FetchListOptions>>(() => ({
      offset: 0,
      limit: 100,
      type: this.selectedType()?.id,
      baseModel: this.selectedBaseModel()?.id
    }));
    this.candidateModelsResource = modelRepository.fetchListResource(options);
  }

  readonly selectableTypes: Signal<SelectOption[]>;
  readonly selectableBaseModels: Signal<SelectOption[]>;

  selectedType = signal<ModelType | null>(null);
  selectedBaseModel = signal<BaseModel | null>(null);

  selectedModel = input<Model | null>(null);
  selectedModelChange = output<Model | null>();

  readonly candidateModelsResource: Resource<MultipleModels>;

  onModelChange(model: Model | null) {
    this.selectedModelChange.emit(model);
  }
}
