import {Injectable} from '@angular/core';
import {HttpClient} from '@angular/common/http';
import {firstValueFrom, map, Observable} from 'rxjs';
import {DecodedPng, MultipleProducts, Product} from '../data/product';
import {PathProvider} from './path-provider';

export interface FetchListOptions {
  offset: number;
  limit: number;
}

interface ProductResponse {
  product: Product;
}

@Injectable({
  providedIn: 'root'
})
export class ProductRepository {

  constructor(private httpClient: HttpClient, private pathProvider: PathProvider) {
  }

  fetchListByModel(modelId: string, options?: Partial<FetchListOptions>): Observable<MultipleProducts> {
    const url = new URL(`${this.pathProvider.getApiUrl()}/v1/models/${modelId}/products`);
    if (options?.offset) {
      url.searchParams.set('offset', options.offset.toString());
    }
    if (options?.limit) {
      url.searchParams.set('limit', options.limit.toString());
    }

    return this.httpClient.get<MultipleProducts>(url.toString());
  }

  fetchList(options?: Partial<FetchListOptions>): Observable<MultipleProducts> {
    const url = new URL(`${this.pathProvider.getApiUrl()}/v1/products`);
    if (options?.offset) {
      url.searchParams.set('offset', options.offset.toString());
    }
    if (options?.limit) {
      url.searchParams.set('limit', options.limit.toString());
    }

    return this.httpClient.get<MultipleProducts>(url.toString());
  }

  fetchById(id: string): Observable<Product> {
    return this.httpClient.get<ProductResponse>(`${this.pathProvider.getApiUrl()}/v1/products/${id}`).pipe(map(response => response.product));
  }

  decode(file: File): Promise<DecodedPng> {
    const formData = new FormData();
    formData.append('file', file);
    return firstValueFrom(this.httpClient.post<DecodedPng>(`${this.pathProvider.getApiUrl()}/v1/png/workflow/decoder`, formData, {}))
  }

  upload(formData: FormData): Observable<Product> {
    return this.httpClient.post<ProductResponse>(`${this.pathProvider.getApiUrl()}/v1/products`, formData).pipe(map(response => response.product));
  }
}
