import {Component, effect, signal} from '@angular/core';
import {MatProgressBar} from "@angular/material/progress-bar";
import {ImageList} from '../../parts/image-list/image-list';
import {map} from 'rxjs';
import {FetchListOptions, ProductRepository} from '../../../apis/repositories/product-repository';
import {ActivatedRoute} from '@angular/router';
import {MultipleProducts, Product} from '../../../apis/data/product';
import {Title} from '@angular/platform-browser';
import {Model} from '../../../apis/data/model';
import {ModelRepository} from '../../../apis/repositories/model-repository';

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
    route.params.pipe(map(params => params['id'])).subscribe(async id => {
      this.model.set(await modelRepository.fetchByIdAsync(id));
    })

    effect(() => {
      const model = this.model();
      const name = model?.name ?? 'Image';
      title.setTitle(`${name} gallery - Diffusion Stash`);
    });

    effect(() => {
      if (this.totalProductCount && this.offset >= this.totalProductCount) {
        return;
      }

      if (!this.isBottomReached()) {
        return;
      }

      const model = this.model();
      if (model === undefined) {
        return;
      }

      const options: FetchListOptions = {
        offset: this.offset,
        limit: ImageGallery.DEFAULT_LIMIT,
      };
      this.assignMore(
        model
          ? productRepository.fetchListByModelAsync(model.id, options)
          : productRepository.fetchListAsync(options)
      ).finally(() => this.isLoading.set(false));
    });

  }

  private offset = 0;
  private totalProductCount: number | null = null;

  isLoading = signal(false);

  private async assignMore(promise: Promise<MultipleProducts | null>) {
    this.isLoading.set(true);
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

  model = signal<Model | null | undefined>(undefined);

  readonly isBottomReached = signal(true);
  readonly totalProducts = signal(0);
  readonly items = signal<Product[]>([]);

  onBottomReach() {
    this.isBottomReached.set(true);
  }
}
