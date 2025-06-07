import {Component} from '@angular/core';
import {FormsModule, NgForm} from '@angular/forms';
import {PickFile} from '../../parts/pick-file/pick-file';
import {MatFormField, MatInput, MatLabel} from '@angular/material/input';
import {MatOption, MatSelect} from '@angular/material/select';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {MatButton} from '@angular/material/button';
import {MatProgressBar} from '@angular/material/progress-bar';
import {HttpEventType, HttpUploadProgressEvent} from '@angular/common/http';
import {MatSnackBar} from '@angular/material/snack-bar';

interface SelectOption {
  value: string | null;
  label: string;
}

type ProcessingState = 'uploading' | 'processing';

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
  constructor(private modelRepository: ModelRepository, private snackBar: MatSnackBar) {
  }

  readonly selectableTypes: SelectOption[] = [
    {value: 'Checkpoint', label: 'Checkpoint'},
    {value: 'Lora', label: 'LoRA'},
    {value: 'Lycoris', label: 'LyCORIS'},
    {value: 'Vae', label: 'VAE'},
    {value: 'Embedding', label: 'Embedding'},
    {value: 'Upscaler', label: 'Upscaler Model'},
    {value: 'ControlNet', label: 'ControlNet Model'},
    {value: null, label: 'Other'},
  ];

  readonly selectableCategories: SelectOption[] = [
    {value: 'StableDiffusion15', label: 'Stable Diffusion 1.5'},
    {value: 'StableDiffusionXl', label: 'Stable Diffusion XL'},
    {value: 'PonyDiffusion', label: 'Pony Diffusion'},
    {value: 'Illustrious', label: 'Illustrious'},
    {value: 'StableDiffusion30', label: 'Stable Diffusion 3'},
    {value: 'StableDiffusion35', label: 'Stable Diffusion 3.5'},
    {value: 'Flux1D', label: 'Flux.1 D'},
    {value: 'Flux1S', label: 'Flux.1 S'},
    {value: null, label: 'Other'},
  ];

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
      formData.append('category', form.value.category);
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
