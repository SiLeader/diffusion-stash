import {Component} from '@angular/core';
import {Observable} from 'rxjs';
import {MultipleModels} from '../../../apis/data/model';
import {AsyncPipe} from '@angular/common';
import {ModelList} from '../../parts/model-list/model-list';
import {MatProgressBar} from '@angular/material/progress-bar';
import {ModelRepository} from '../../../apis/repositories/model-repository';

@Component({
  selector: 'app-model-gallery',
  imports: [
    AsyncPipe,
    ModelList,
    MatProgressBar,
  ],
  templateUrl: './model-gallery.html',
  styleUrl: './model-gallery.css'
})
export class ModelGallery {
  constructor(modelRepository: ModelRepository) {
    this.models = modelRepository.fetchList({offset: 0, limit: 100});
  }

  models: Observable<MultipleModels>;
}
