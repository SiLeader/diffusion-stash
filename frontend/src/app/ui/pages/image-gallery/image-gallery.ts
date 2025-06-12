import {Component, effect, Resource} from '@angular/core';
import {MatProgressBar} from "@angular/material/progress-bar";
import {ImageList} from '../../parts/image-list/image-list';
import {map} from 'rxjs';
import {ProductRepository} from '../../../apis/repositories/product-repository';
import {ActivatedRoute} from '@angular/router';
import {MultipleProducts} from '../../../apis/data/product';
import {Title} from '@angular/platform-browser';
import {Model} from '../../../apis/data/model';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {toSignal} from '@angular/core/rxjs-interop';

@Component({
  selector: 'app-image-gallery',
  imports: [
    MatProgressBar,
    ImageList
  ],
  templateUrl: './image-gallery.html',
  styleUrl: './image-gallery.css'
})
export class ImageGallery {
  constructor(route: ActivatedRoute, productRepository: ProductRepository, modelRepository: ModelRepository, title: Title) {
    title.setTitle('Image Gallery - Diffusion Stash');
    const id = toSignal(route.params.pipe(map(params => params['id'])));

    this.products = productRepository.fetchListByModelOrWhole(id);
    this.model = modelRepository.fetchByIdOrNull(id);

    effect(() => {
      title.setTitle(`${this.model.value()?.name} gallery - Diffusion Stash`);
    });
  }

  model: Resource<Model | null>;
  products: Resource<MultipleProducts | null>;
}
