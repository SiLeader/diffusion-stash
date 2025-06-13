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
      if (this.totalProductCount && this.offset + ImageGallery.DEFAULT_LIMIT >= this.totalProductCount) {
        return;
      }

      if (!this.isBottomReached()) {
        return;
      }

      const model = this.model.value();
      const options: FetchListOptions = {
        offset: this.offset,
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

  private offset = 0;
  private totalProductCount: number | null = null;

  private async assignMore(promise: Promise<MultipleProducts | null>) {
    const loaded = await promise;
    if (!loaded) {
      return;
    }

    this.items.update(items => [...items, ...loaded.products]);
    this.totalProducts.set(loaded.total);
    this.isBottomReached.set(false);
    this.totalProductCount = loaded.total;
    this.offset += loaded.products.length;
  }

  model: Resource<Model | null>;

  readonly isBottomReached = signal(false);

  readonly totalProducts = signal(0);

  readonly items = signal<Product[]>([]);

  onBottomReach() {
    this.isBottomReached.set(true);
  }
}
