import {Component, Signal} from '@angular/core';
import {FormsModule, NgForm} from '@angular/forms';
import {PickFile} from '../../parts/pick-file/pick-file';
import {MatFormField, MatInput, MatLabel} from '@angular/material/input';
import {MatOption, MatSelect} from '@angular/material/select';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {MatButton} from '@angular/material/button';
import {MatProgressBar} from '@angular/material/progress-bar';
import {HttpEventType, HttpUploadProgressEvent} from '@angular/common/http';
import {MatSnackBar} from '@angular/material/snack-bar';
import {ModelCategoryProvider} from '../../../apis/repositories/model-category-provider';
import {SelectOption} from '../../../apis/data/model-category';

@Component({
  selector: 'app-upload-model',
  imports: [
    FormsModule,
    PickFile,
    MatFormField,
    MatLabel,
    MatInput,
    MatFormField,
    MatSelect,
    MatOption,
    MatButton,
    MatProgressBar,
  ],
  templateUrl: './upload-model.html',
  styleUrl: './upload-model.css'
})
export class UploadModel {
  constructor(
    private modelRepository: ModelRepository,
    private snackBar: MatSnackBar,
    modelCategoryProvider: ModelCategoryProvider
  ) {
    this.selectableTypes = modelCategoryProvider.fetchSelectableModelTypes(modelCategoryProvider.fetchModelTypes());
    this.selectableBaseModels = modelCategoryProvider.fetchSelectableBaseModels(modelCategoryProvider.fetchBaseModels());
  }

  readonly selectableTypes: Signal<SelectOption[]>;
  readonly selectableBaseModels: Signal<SelectOption[]>;

  progress: number | null = null;
  selectedFile: File | null = null;

  onSubmit(form: NgForm) {
    if (form.invalid || !this.selectedFile) {
      this.snackBar.open('Please fill all fields', 'OK');
      return;
    }

    const formData = new FormData();
    formData.append('file', this.selectedFile);
    formData.append('name', form.value.name);
    formData.append('description', form.value.description);
    if (form.value.category) {
      formData.append('base_model', form.value.category);
    }
    if (form.value.type) {
      formData.append('model_type', form.value.type);
    }

    this.modelRepository.upload(formData).subscribe(event => {
      if (event.type === HttpEventType.UploadProgress) {
        const e = event as HttpUploadProgressEvent;
        this.progress = Math.round(100 * e.loaded / (e.total || 100));
      } else if (event.type === HttpEventType.Response) {
        this.progress = null;
        this.snackBar.open('Model uploaded successfully', 'OK');
      }
    });
  }
}
