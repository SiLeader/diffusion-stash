import {Component, input} from '@angular/core';
import {Model} from '../../../apis/data/model';

@Component({
  selector: 'app-model-info-table',
  imports: [],
  templateUrl: './model-info-table.html',
  styleUrl: './model-info-table.css'
})
export class ModelInfoTable {
  model = input.required<Model>();
}
