import {Component, OnInit} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {ActivatedRoute} from '@angular/router';
import {map, mergeAll, Observable} from 'rxjs';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {AsyncPipe, NgOptimizedImage} from '@angular/common';
import {MatProgressBar} from '@angular/material/progress-bar';
import {ModelInfoTable} from '../../parts/model-info-table/model-info-table';
import {DefaultImage} from '../../directive/default-image';
import {MatCardImage} from '@angular/material/card';
import {ThumbnailPipe} from '../../pipe/thumbnail-pipe';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {ModelContentPipe} from '../../pipe/model-content-pipe';

@Component({
  selector: 'app-model-detail',
  imports: [
    AsyncPipe,
    MatProgressBar,
    ModelInfoTable,
    DefaultImage,
    MatCardImage,
    ThumbnailPipe,
    MatIcon,
    MatButton,
    ModelContentPipe
  ],
  templateUrl: './model-detail.html',
  styleUrl: './model-detail.css'
})
export class ModelDetail implements OnInit {
  constructor(route: ActivatedRoute, modelRepository: ModelRepository) {
    this.model = route.params.pipe(map(params => modelRepository.fetchById(params['id'])), mergeAll())
  }

  containerClass = 'container-xs';

  ngOnInit() {
    this.setContainerClass(window.innerWidth);
  }

  onResize(event: Event) {
    this.setContainerClass((<any>event.target).innerWidth);
  }

  setContainerClass(width: number) {
    if (width <= 600) {
      this.containerClass = 'container-xs';
    } else if (width <= 960) {
      this.containerClass = 'container-sm';
    } else if (width <= 1280) {
      this.containerClass = 'container-md';
    } else {
      this.containerClass = 'container-lg';
    }
  }

  model: Observable<Model>;
}
