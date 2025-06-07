import {Component, computed, EventEmitter, input, Output} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {
  MatCard, MatCardActions,
  MatCardContent,
  MatCardFooter,
  MatCardHeader, MatCardImage,
  MatCardSubtitle,
  MatCardTitle
} from '@angular/material/card';
import {environment} from '../../../../environments/environment';
import {NgOptimizedImage} from '@angular/common';
import {DefaultImage} from '../../directive/default-image';
import {MatButton} from '@angular/material/button';

@Component({
  selector: 'app-model-card',
  imports: [
    MatCard,
    MatCardTitle,
    MatCardHeader,
    MatCardSubtitle,
    MatCardFooter,
    MatCardContent,
    NgOptimizedImage,
    MatCardImage,
    DefaultImage,
    MatCardActions,
    MatButton
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
