import {Component, input} from '@angular/core';
import {Product} from '../../../apis/data/product';
import {MatList, MatListItem, MatListItemLine, MatListItemTitle} from '@angular/material/list';

@Component({
  selector: 'app-image-info',
  imports: [
    MatList,
    MatListItem,
    MatListItemTitle,
    MatListItemLine
  ],
  templateUrl: './image-info.html',
  styleUrl: './image-info.css'
})
export class ImageInfo {
  product = input.required<Product>();
}
