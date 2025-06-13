import {Component, effect, signal} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {ModelList} from '../../parts/model-list/model-list';
import {MatProgressBar} from '@angular/material/progress-bar';
import {ModelRepository} from '../../../apis/repositories/model-repository';
import {Title} from '@angular/platform-browser';
import {FetchListOptions} from '../../../apis/repositories/product-repository';

@Component({
  selector: 'app-model-gallery',
  imports: [
    ModelList,
    MatProgressBar,
  ],
  templateUrl: './model-gallery.html',
  styleUrl: './model-gallery.css'
})
export class ModelGallery {
  private static readonly DEFAULT_LIMIT = 20;

  constructor(private readonly modelRepository: ModelRepository, title: Title) {
    title.setTitle('Models - Diffusion Stash');

    effect(() => {
      if (this.totalProductCount && this.offset >= this.totalProductCount) {
        return;
      }

      if (!this.isBottomReached()) {
        return;
      }

      this.assignMore().finally(() => this.isLoading.set(false));
    });
  }

  private async assignMore() {
    this.isLoading.set(true);
    const options: FetchListOptions = {
      offset: this.offset,
      limit: ModelGallery.DEFAULT_LIMIT,
    };
    const loaded = await this.modelRepository.fetchListAsync(options);
    if (!loaded) {
      return;
    }

    this.items.update(items => [...items, ...loaded.models]);
    this.totalModels.set(loaded.total);
    this.isBottomReached.set(false);
    this.totalProductCount = loaded.total;
    this.offset += loaded.models.length;
  }

  private offset = 0;
  private totalProductCount: number | null = null;

  isLoading = signal(false);

  readonly isBottomReached = signal(true);
  readonly totalModels = signal(0);
  readonly items = signal<Model[]>([]);

  onBottomReach() {
    this.isBottomReached.set(true);
  }
}
