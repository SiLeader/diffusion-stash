import {Component} from '@angular/core';
import {MatToolbar} from '@angular/material/toolbar';
import {MatButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';

@Component({
  selector: 'app-app-bar',
  imports: [
    MatToolbar,
    MatButton,
    MatIcon
  ],
  templateUrl: './app-bar.html',
  styleUrl: './app-bar.css'
})
export class AppBar {

}
