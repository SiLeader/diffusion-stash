import {Injectable, Resource, Signal} from '@angular/core';
import {HttpClient, HttpEvent, httpResource} from '@angular/common/http';
import {map, Observable} from 'rxjs';
import {Model, MultipleModels} from '../data/model';
import {PathProvider} from './path-provider';

export interface FetchListOptions {
  offset: number;
  limit: number;
  query: string;
  baseModel: string;
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

  private listUrl(options?: Partial<FetchListOptions>): string {
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
    if (options?.baseModel) {
      url.searchParams.set('category', options.baseModel);
    }
    if (options?.type) {
      url.searchParams.set('type', options.type);
    }
    return url.toString();
  }

  fetchList(options?: Partial<FetchListOptions>): Observable<MultipleModels> {
    return this.httpClient.get<MultipleModels>(this.listUrl(options));
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

  fetchListResource(): Resource<MultipleModels> {
    return httpResource(
      () => ({
        url: this.listUrl()
      }),
      {
        defaultValue: {models: [], total: 0},
      }
    );
  }
}
