import {Component} from '@angular/core';
import {AsyncPipe} from "@angular/common";
import {MatProgressBar} from "@angular/material/progress-bar";
import {ImageList} from '../../parts/image-list/image-list';
import {map, mergeAll, Observable} from 'rxjs';
import {ProductRepository} from '../../../apis/repositories/product-repository';
import {ActivatedRoute} from '@angular/router';
import {MultipleProducts} from '../../../apis/data/product';

@Component({
  selector: 'app-image-gallery',
  imports: [
    AsyncPipe,
    MatProgressBar,
    ImageList
  ],
  templateUrl: './image-gallery.html',
  styleUrl: './image-gallery.css'
})
export class ImageGallery {
  constructor(route: ActivatedRoute, productRepository: ProductRepository) {
    this.products = route.params.pipe(map(params => {
      const id = params['id'];
      if (id) {
        return productRepository.fetchListByModel(id, {
          offset: 0,
          limit: 100
        })
      } else {
        return productRepository.fetchList({
          offset: 0,
          limit: 100
        })
      }
    }), mergeAll());
  }

  products: Observable<MultipleProducts>;
}
