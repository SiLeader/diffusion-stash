import {Component, computed, EventEmitter, input, Output} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {
  MatCard, MatCardActions,
  MatCardContent, MatCardImage,
  MatCardTitle
} from '@angular/material/card';
import {environment} from '../../../../environments/environment';
import {DefaultImage} from '../../directive/default-image';
import {MatButton} from '@angular/material/button';
import {ThumbnailPipe} from '../../pipe/thumbnail-pipe';

@Component({
  selector: 'app-model-card',
  imports: [
    MatCard,
    MatCardTitle,
    MatCardContent,
    MatCardImage,
    DefaultImage,
    MatCardActions,
    MatButton,
    ThumbnailPipe
  ],
  templateUrl: './model-card.html',
  styleUrl: './model-card.css'
})
export class ModelCard {
  model = input.required<Model>();

  @Output()
  click: EventEmitter<Model> = new EventEmitter();

  thumbnailUrl = computed(() => `${environment.apiUrl}/v1/models/${this.model().id}/thumbnail`);
}
