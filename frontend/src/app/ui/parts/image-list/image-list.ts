import {ChangeDetectorRef, Component, effect, input, output, ViewChild} from '@angular/core';
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
  constructor(cdr: ChangeDetectorRef) {
    effect(() => {
      const products = this.products();
      const total = this.total();
      if (products) {
        this.updateTrigger = !this.updateTrigger;
        cdr.detectChanges();
        this.updateTrigger = !this.updateTrigger;
        if (products.length === total) {
          this.masonry.layout();
        }
      }
    });
  }

  @ViewChild(NgxMasonryComponent) masonry!: NgxMasonryComponent;

  total = input.required<number>();
  products = input.required<Product[]>();

  updateTrigger = false;

  bottomReach = output();

  masonryOptions: NgxMasonryOptions = {
    itemSelector: '.image-item',
    columnWidth: '.image-item',
    horizontalOrder: true,
    percentPosition: true,
    animations: {},
    gutter: 12,
  };
}
