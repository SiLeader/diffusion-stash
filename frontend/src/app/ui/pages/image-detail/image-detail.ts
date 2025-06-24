import {Component, effect, Resource} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {Title} from '@angular/platform-browser';
import {toSignal} from '@angular/core/rxjs-interop';
import {map} from 'rxjs';
import {Product} from '../../../apis/data/product';
import {ProductRepository} from '../../../apis/repositories/product-repository';
import {DefaultImage} from '../../directive/default-image';
import {MatCardImage} from '@angular/material/card';
import {ProductContentPipe} from '../../pipe/product-content-pipe';
import {ImageInfo} from '../../parts/image-info/image-info';

@Component({
  selector: 'app-image-detail',
  imports: [
    DefaultImage,
    MatCardImage,
    ProductContentPipe,
    ImageInfo
  ],
  templateUrl: './image-detail.html',
  styleUrl: './image-detail.css'
})
export class ImageDetail {
  constructor(route: ActivatedRoute, productRepository: ProductRepository, title: Title) {
    const id = toSignal(route.params.pipe(map(params => params['id'])));

    this.product = productRepository.fetchById(id);
    title.setTitle('Image Detail - Diffusion Stash');
    effect(() => {
      title.setTitle(`${this.product.value()?.name} - Diffusion Stash`);
    });
  }

  product: Resource<Product | null>;
}
