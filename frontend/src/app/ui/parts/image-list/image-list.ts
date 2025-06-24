import {Component, effect, input, output, ViewChild} from '@angular/core';
import {RouterLink} from '@angular/router';
import {Product} from '../../../apis/data/product';
import {ProductContentPipe} from '../../pipe/product-content-pipe';
import {InfiniteScrollDirective} from 'ngx-infinite-scroll';
import {NgxMasonryComponent, NgxMasonryModule, NgxMasonryOptions} from 'ngx-masonry';

@Component({
  selector: 'app-image-list',
  imports: [
    ProductContentPipe,
    InfiniteScrollDirective,
    RouterLink,
    NgxMasonryModule,
  ],
  templateUrl: './image-list.html',
  styleUrl: './image-list.css'
})
export class ImageList {
  constructor() {
    effect(() => {
      const products = this.products();
      if (products) {
        setTimeout(() => {
          if (this.masonry) {
            console.log(products);
            this.masonry.layout();
          }
        }, 2000);
      }
    });
  }

  @ViewChild(NgxMasonryComponent) masonry?: NgxMasonryComponent;

  total = input.required<number>();
  products = input.required<Product[]>();

  bottomReach = output();

  masonryOptions: NgxMasonryOptions = {
    itemSelector: '.image-item',
    columnWidth: '.grid-item',
    horizontalOrder: true,
    percentPosition: true,
    animations: {},
    gutter: 12,
  };
}
