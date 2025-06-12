import {Component, input, output, Resource} from '@angular/core';
import {MatFormField, MatOption, MatSelect} from '@angular/material/select';
import {Model, MultipleModels} from '../../../apis/data/model';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {FormsModule} from '@angular/forms';
import {MatLabel} from '@angular/material/input';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';

@Component({
  selector: 'app-model-selector',
  imports: [
    MatSelect,
    MatFormField,
    MatOption,
    FormsModule,
    MatLabel,
    MatButton,
    MatIcon
  ],
  templateUrl: './model-selector.html',
  styleUrl: './model-selector.css'
})
export class ModelSelector {
  constructor(
    modelRepository: ModelRepository,
  ) {
    this.candidateModelsResource = modelRepository.fetchListResource();
  }

  selectedModel = input<Model | null>(null);
  selectedModelChange = output<Model | null>();
  modelRemove = output();

  readonly candidateModelsResource: Resource<MultipleModels>;

  onModelChange(modelId: string | null) {
    let model = null;
    if (modelId) {
      model = this.candidateModelsResource.value().models.find(m => m.id === modelId) ?? null;
    }
    this.selectedModelChange.emit(model);
  }

  onModelRemove() {
    this.modelRemove.emit();
  }
}
