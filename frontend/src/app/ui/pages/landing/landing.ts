import {Component} from '@angular/core';
import {Observable} from 'rxjs';
import {Model, MultipleModels} from '../../../apis/data/model';
import {AsyncPipe} from '@angular/common';
import {ModelList} from '../../parts/model-list/model-list';
import {MatProgressBar} from '@angular/material/progress-bar';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {MatButton} from '@angular/material/button';

@Component({
  selector: 'app-landing',
  imports: [
    AsyncPipe,
    ModelList,
    MatProgressBar,
    MatButton
  ],
  templateUrl: './landing.html',
  styleUrl: './landing.css'
})
export class Landing {
  constructor(modelRepository: ModelRepository) {
    this.models = modelRepository.fetchList({offset: 0, limit: 100});
  }

  models: Observable<MultipleModels>;
}
