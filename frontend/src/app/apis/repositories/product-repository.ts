import {Injectable, Resource, Signal} from '@angular/core';
import {HttpClient, httpResource} from '@angular/common/http';
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

  private createUrl(path: string, options?: Partial<FetchListOptions>): string {
    const url = new URL(`${this.pathProvider.getApiUrl()}${path}`);
    if (options?.offset) {
      url.searchParams.set('offset', options.offset.toString());
    }
    if (options?.limit) {
      url.searchParams.set('limit', options.limit.toString());
    }
    return url.toString();
  }

  async fetchListByModelAsync(modelId: string, options?: Partial<FetchListOptions>): Promise<MultipleProducts | null> {
    const res = await fetch(this.createUrl(`/v1/models/${modelId}/products`, options));
    if (!res.ok) {
      return null;
    }

    const data = await res.json();
    return data as MultipleProducts;
  }

  async fetchListAsync(options?: Partial<FetchListOptions>): Promise<MultipleProducts | null> {
    const res = await fetch(this.createUrl(`/v1/products`, options));
    if (!res.ok) {
      return null;
    }

    const data = await res.json();
    return data as MultipleProducts;
  }

  fetchListByModelOrWhole(modelId: Signal<string | null | undefined>, options?: Signal<Partial<FetchListOptions>>): Resource<MultipleProducts | null> {
    return httpResource<MultipleProducts | null>(
      () => {
        const mid = modelId();
        const opts = options ? options() : undefined;
        return {
          url: this.createUrl(mid ? `/v1/models/${mid}/products` : '/v1/products', opts),
        }
      },
      {
        defaultValue: null,
      }
    );
  }

  fetchById(id: Signal<string>): Resource<Product | null> {
    return httpResource<Product | null>(
      () => ({
        url: `${this.pathProvider.getApiUrl()}/v1/products/${id()}`
      }),
      {
        defaultValue: null,
        parse: (response: unknown) => (response as ProductResponse).product
      }
    );
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
