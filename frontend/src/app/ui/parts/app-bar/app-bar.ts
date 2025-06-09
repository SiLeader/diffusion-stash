import {MatToolbar, MatToolbarRow} from '@angular/material/toolbar';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {RouterLink} from '@angular/router';
import {Component} from '@angular/core';

@Component({
  selector: 'app-app-bar',
  imports: [
    MatToolbar,
    MatButton,
    MatIcon,
    RouterLink,
    MatToolbarRow,
  ],
  templateUrl: './app-bar.html',
  styleUrl: './app-bar.css'
})
export class AppBar {
}
