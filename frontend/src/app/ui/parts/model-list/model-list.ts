import {Component, input, OnInit, output} from '@angular/core';
import {Model} from '../../../apis/data/model';
import {MatGridList, MatGridTile} from '@angular/material/grid-list';
import {ModelCard} from '../model-card/model-card';
import {Router} from '@angular/router';
import {InfiniteScrollDirective} from 'ngx-infinite-scroll';

@Component({
  selector: 'app-model-list',
  imports: [
    MatGridList,
    MatGridTile,
    ModelCard,
    InfiniteScrollDirective
  ],
  templateUrl: './model-list.html',
  styleUrl: './model-list.css'
})
export class ModelList implements OnInit {
  constructor(private router: Router) {
  }

  total = input.required<number>();
  models = input.required<Model[]>();

  bottomReach = output();

  columns = 4;

  ngOnInit() {
    this.setColumnsFromWidth(window.innerWidth);
  }

  async onClick(model: Model) {
    await this.router.navigate(['/models', model.id]);
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
