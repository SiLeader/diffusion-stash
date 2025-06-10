import {Injectable} from '@angular/core';
import {HttpClient, HttpEvent} from '@angular/common/http';
import {map, Observable} from 'rxjs';
import {Model, MultipleModels} from '../data/model';
import {PathProvider} from './path-provider';

export interface FetchListOptions {
  offset: number;
  limit: number;
  query: string;
  category: string;
  type: string;
}

interface ModelResponse {
  model: Model;
}

@Injectable({
  providedIn: 'root'
})
export class ModelRepository {

  constructor(private httpClient: HttpClient, private pathProvider: PathProvider) {
  }

  fetchList(options?: Partial<FetchListOptions>): Observable<MultipleModels> {
    const url = new URL(`${this.pathProvider.getApiUrl()}/v1/models`);
    if (options?.offset) {
      url.searchParams.set('offset', options.offset.toString());
    }
    if (options?.limit) {
      url.searchParams.set('limit', options.limit.toString());
    }
    if (options?.query) {
      url.searchParams.set('query', options.query);
    }
    if (options?.category) {
      url.searchParams.set('category', options.category);
    }
    if (options?.type) {
      url.searchParams.set('type', options.type);
    }
    return this.httpClient.get<MultipleModels>(url.toString());
  }

  fetchById(id: string): Observable<Model> {
    return this.httpClient.get<ModelResponse>(`${this.pathProvider.getApiUrl()}/v1/models/${id}`).pipe(map(response => response.model));
  }

  upload(formData: FormData): Observable<HttpEvent<any>> {
    return this.httpClient.post<ModelResponse>(`${this.pathProvider.getApiUrl()}/v1/models`, formData, {
      reportProgress: true,
      observe: 'events'
    });
  }
}
