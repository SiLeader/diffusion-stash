import {Component, input} from '@angular/core';
import {Product} from '../../../apis/data/product';
import {MatDivider, MatList, MatListItem, MatListItemLine, MatListItemTitle} from '@angular/material/list';
import {MatIcon} from '@angular/material/icon';
import {MatButton} from '@angular/material/button';
import {Clipboard} from '@angular/cdk/clipboard';
import {MatSnackBar} from '@angular/material/snack-bar';

@Component({
  selector: 'app-image-info',
  imports: [
    MatList,
    MatListItem,
    MatListItemTitle,
    MatListItemLine,
    MatIcon,
    MatButton,
    MatDivider,
  ],
  templateUrl: './image-info.html',
  styleUrl: './image-info.css'
})
export class ImageInfo {
  constructor(private clipboard: Clipboard, private snackBar: MatSnackBar) {
  }

  product = input.required<Product>();

  copyPositivePrompt(prompt: string) {
    this.copy(prompt);
  }

  copyNegativePrompt(prompt: string) {
    this.copy(prompt);
  }

  private copy(value: string) {
    this.clipboard.copy(value);
    this.snackBar.open('Copied!', 'OK');

  }
}
