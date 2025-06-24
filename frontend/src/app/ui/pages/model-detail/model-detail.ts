import {Component, effect, Resource} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {ActivatedRoute} from '@angular/router';
import {map} from 'rxjs';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {MatProgressBar} from '@angular/material/progress-bar';
import {ModelInfoTable} from '../../parts/model-info-table/model-info-table';
import {DefaultImage} from '../../directive/default-image';
import {MatCardImage} from '@angular/material/card';
import {ThumbnailPipe} from '../../pipe/thumbnail-pipe';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {ModelContentPipe} from '../../pipe/model-content-pipe';
import {Title} from '@angular/platform-browser';
import {toSignal} from '@angular/core/rxjs-interop';

@Component({
  selector: 'app-model-detail',
  imports: [
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
export class ModelDetail {
  constructor(route: ActivatedRoute, modelRepository: ModelRepository, title: Title) {
    const id = toSignal(route.params.pipe(map(params => params['id'])));
    this.model = modelRepository.fetchById(id);
    title.setTitle('Model Detail - Diffusion Stash');

    effect(() => {
      title.setTitle(`${this.model.value()?.name} - Diffusion Stash`);
    });
  }

  model: Resource<Model | null>;
}
