import {Injectable} from '@angular/core';
import {environment} from '../../../environments/environment';

@Injectable({
  providedIn: 'root'
})
export class PathProvider {

  constructor() {
  }

  getApiUrl(): string {
    const url = environment.apiUrl;
    if (url.length > 0) {
      return url;
    } else {
      return location.origin;
    }
  }
}
