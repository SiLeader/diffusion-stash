import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';
import {AppBar} from './ui/parts/app-bar/app-bar';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, AppBar],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App {
  protected title = 'frontend';
}
