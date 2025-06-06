import {Component} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {ActivatedRoute} from '@angular/router';
import {map, mergeAll, Observable} from 'rxjs';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {AsyncPipe} from '@angular/common';
import {MatButton} from '@angular/material/button';
import {MatProgressBar} from '@angular/material/progress-bar';
import {ModelList} from '../../parts/model-list/model-list';

@Component({
  selector: 'app-model-detail',
  imports: [
    AsyncPipe,
    MatButton,
    MatProgressBar,
    ModelList
  ],
  templateUrl: './model-detail.html',
  styleUrl: './model-detail.css'
})
export class ModelDetail {
  constructor(route: ActivatedRoute, modelRepository: ModelRepository) {
    this.model = route.params.pipe(map(params => modelRepository.fetchById(params['id'])), mergeAll())
  }

  model: Observable<Model>;
}
