import {Component, input, OnInit, output} from '@angular/core';
import {MatGridList, MatGridTile} from "@angular/material/grid-list";
import {Router, RouterLink} from '@angular/router';
import {Product} from '../../../apis/data/product';
import {ProductContentPipe} from '../../pipe/product-content-pipe';
import {InfiniteScrollDirective} from 'ngx-infinite-scroll';
import {MatProgressSpinner} from '@angular/material/progress-spinner';

@Component({
  selector: 'app-image-list',
  imports: [
    MatGridList,
    MatGridTile,
    ProductContentPipe,
    InfiniteScrollDirective,
    MatProgressSpinner,
    RouterLink,
  ],
  templateUrl: './image-list.html',
  styleUrl: './image-list.css'
})
export class ImageList implements OnInit {
  constructor(private router: Router) {
  }

  total = input.required<number>();
  products = input.required<Product[]>();
  isLoading = input(false);

  bottomReach = output();

  columns = 4;

  ngOnInit() {
    this.setColumnsFromWidth(window.innerWidth);
  }

  async onClick(product: Product) {
    await this.router.navigate(['/products', product.id]);
  }

  onResize(event: Event) {
    this.setColumnsFromWidth((<any>event.target).innerWidth);
  }

  private setColumnsFromWidth(width: number) {
    if (width <= 600) {
      this.columns = 1;
    } else if (width <= 960) {
      this.columns = 2;
    } else if (width <= 1280) {
      this.columns = 3;
    } else {
      this.columns = 4;
    }
  }
}
