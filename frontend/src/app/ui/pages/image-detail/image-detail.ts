import {Component, effect, OnInit, Resource} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {Title} from '@angular/platform-browser';
import {toSignal} from '@angular/core/rxjs-interop';
import {map} from 'rxjs';
import {Product} from '../../../apis/data/product';
import {ProductRepository} from '../../../apis/repositories/product-repository';
import {DefaultImage} from '../../directive/default-image';
import {MatCardImage} from '@angular/material/card';
import {ThumbnailPipe} from '../../pipe/thumbnail-pipe';
import {ProductContentPipe} from '../../pipe/product-content-pipe';
import {ImageInfo} from '../../parts/image-info/image-info';

@Component({
  selector: 'app-image-detail',
  imports: [
    DefaultImage,
    MatCardImage,
    ThumbnailPipe,
    ProductContentPipe,
    ImageInfo
  ],
  templateUrl: './image-detail.html',
  styleUrl: './image-detail.css'
})
export class ImageDetail implements OnInit {
  constructor(route: ActivatedRoute, productRepository: ProductRepository, title: Title) {
    const id = toSignal(route.params.pipe(map(params => params['id'])));

    this.product = productRepository.fetchById(id);
    title.setTitle('Image Detail - Diffusion Stash');
    effect(() => {
      title.setTitle(`${this.product.value()?.name} - Diffusion Stash`);
    });
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

  product: Resource<Product | null>;
}
