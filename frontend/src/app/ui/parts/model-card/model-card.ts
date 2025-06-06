import {Component, EventEmitter, input, Output} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {MatCard, MatCardHeader, MatCardSubtitle, MatCardTitle} from '@angular/material/card';

@Component({
  selector: 'app-model-card',
  imports: [
    MatCard,
    MatCardTitle,
    MatCardHeader,
    MatCardSubtitle
  ],
  templateUrl: './model-card.html',
  styleUrl: './model-card.css'
})
export class ModelCard {
  model = input.required<Model>();

  @Output()
  click: EventEmitter<Model> = new EventEmitter();
}
