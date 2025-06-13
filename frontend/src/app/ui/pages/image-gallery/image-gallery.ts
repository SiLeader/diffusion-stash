import {Component, effect, Resource, signal} from '@angular/core';
import {MatProgressBar} from "@angular/material/progress-bar";
import {ImageList} from '../../parts/image-list/image-list';
import {map} from 'rxjs';
import {FetchListOptions, ProductRepository} from '../../../apis/repositories/product-repository';
import {ActivatedRoute} from '@angular/router';
import {MultipleProducts, Product} from '../../../apis/data/product';
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
  private static readonly DEFAULT_LIMIT = 20;

  constructor(route: ActivatedRoute, productRepository: ProductRepository, modelRepository: ModelRepository, title: Title) {
    title.setTitle('Image Gallery - Diffusion Stash');
    const id = toSignal(route.params.pipe(map(params => params['id'])));

    this.model = modelRepository.fetchByIdOrNull(id);

    effect(() => {
      const offset = this.items().length;
      const total = this.totalProducts();
      if (offset + ImageGallery.DEFAULT_LIMIT >= total) {
        return;
      }

      const model = this.model.value();
      const options: FetchListOptions = {
        offset: offset,
        limit: ImageGallery.DEFAULT_LIMIT,
      };
      this.assignMore(
        model
          ? productRepository.fetchListByModelAsync(model.id, options)
          : productRepository.fetchListAsync(options)
      ).then();
    });

    effect(() => {
      const model = this.model.value();
      const name = model?.name ?? 'Image';
      title.setTitle(`${name} gallery - Diffusion Stash`);
    });
  }

  private async assignMore(promise: Promise<MultipleProducts | null>) {
    const loaded = await promise;
    if (!loaded) {
      return;
    }

    this.items.update(items => [...items, ...loaded.products]);
  }

  model: Resource<Model | null>;

  readonly totalProducts = signal(0);

  readonly items = signal<Product[]>([]);
}
