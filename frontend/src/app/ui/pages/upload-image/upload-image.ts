import {Component, signal} from '@angular/core';
import {FormsModule, NgForm} from '@angular/forms';
import {PickFile} from '../../parts/pick-file/pick-file';
import {MatFormField, MatInput, MatLabel} from '@angular/material/input';
import {MatButton} from '@angular/material/button';
import {ProductRepository} from '../../../apis/repositories/product-repository';
import {MatSnackBar} from '@angular/material/snack-bar';
import {MatIcon} from '@angular/material/icon';
import {MatDivider} from '@angular/material/divider';
import {firstValueFrom} from 'rxjs';
import {Model} from '../../../apis/data/model';
import {MatList, MatListItem} from '@angular/material/list';
import {ModelSelector} from '../../parts/model-selector/model-selector';

@Component({
  selector: 'app-upload-image',
  imports: [
    FormsModule,
    PickFile,
    MatFormField,
    MatLabel,
    MatInput,
    MatIcon,
    MatButton,
    MatDivider,
    MatList,
    MatListItem,
    ModelSelector
  ],
  templateUrl: './upload-image.html',
  styleUrl: './upload-image.css'
})
export class UploadImage {
  constructor(
    private productRepository: ProductRepository,
    private snackBar: MatSnackBar,
  ) {
  }

  selectedFile: File | null = null;
  isProcessing = false;
  models = signal<(Model | null)[]>([]);
  unresolvedModels: string[] = [];

  async onSubmit(form: NgForm) {
    if (!this.selectedFile) {
      this.snackBar.open('Please select a file', 'OK');
      return;
    }

    this.isProcessing = true;

    try {
      const formData = new FormData();
      formData.append("file", this.selectedFile);
      formData.append("positive_prompt", form.value.positive_prompt);
      formData.append("negative_prompt", form.value.negative_prompt);
      formData.append("sampler_name", form.value.sampler_name);
      formData.append("scheduler_name", form.value.scheduler_name);
      formData.append("step_count", form.value.step_count);
      formData.append("cfg_scale", form.value.cfg_scale);
      formData.append("seed", form.value.seed);
      for (const model of this.models()) {
        if (model) {
          formData.append("models", model.id);
        }
      }

      await firstValueFrom(this.productRepository.upload(formData));
      this.snackBar.open('Image uploaded successfully', 'OK');
    } catch {
      this.snackBar.open('Image upload failed', 'OK');
    } finally {
      this.isProcessing = false;
    }
  }

  async onDecode(form: NgForm) {
    if (!this.selectedFile) {
      this.snackBar.open('Please select a PNG file', 'OK');
      return;
    }
    if (this.selectedFile.type !== 'image/png') {
      this.snackBar.open('This feature allows for PNG file only', 'OK');
      return;
    }

    this.isProcessing = true;

    try {
      const decoded = await this.productRepository.decode(this.selectedFile);
      this.models.set(decoded.resolvedModels);
      this.unresolvedModels = decoded.decoded.modelNames.filter(name => !decoded.resolvedModels.find(model => model.fileName === name));
      if (decoded.decoded.positivePrompt) {
        form.controls['positive_prompt'].setValue(decoded.decoded.positivePrompt);
      }
      if (decoded.decoded.negativePrompt) {
        form.controls['negative_prompt'].setValue(decoded.decoded.negativePrompt);
      }
      if (decoded.decoded.samplerName) {
        form.controls['sampler_name'].setValue(decoded.decoded.samplerName);
      }
      if (decoded.decoded.schedulerName) {
        form.controls['scheduler_name'].setValue(decoded.decoded.schedulerName);
      }
      if (decoded.decoded.stepCount) {
        form.controls['step_count'].setValue(decoded.decoded.stepCount);
      }
      if (decoded.decoded.cfgScale) {
        form.controls['cfg_scale'].setValue(decoded.decoded.cfgScale);
      }
      if (decoded.decoded.seed) {
        form.controls['seed'].setValue(decoded.decoded.seed);
      }
    } finally {
      this.isProcessing = false;
    }
  }

  trackKey(index: number, model: Model | null) {
    return model?.id ?? index;
  }

  onAddModel() {
    this.models.update(models => [...models, null]);
  }

  onModelChange(model: Model | null, index: number) {
    console.log(model, index);
    this.models.update(models => {
      models[index] = model;
      return models;
    });
  }

  onModelRemove(index: number) {
    this.models.update(models => {
      models.splice(index, 1);
      return models;
    });
  }
}
