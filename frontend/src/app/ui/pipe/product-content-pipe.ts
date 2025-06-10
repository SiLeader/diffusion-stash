import {Pipe, PipeTransform} from '@angular/core';
import {Product} from '../../apis/data/product';
import {PathProvider} from '../../apis/repositories/path-provider';

@Pipe({
  name: 'productContent'
})
export class ProductContentPipe implements PipeTransform {
  constructor(private pathProvider: PathProvider) {
  }

  transform(value: Product): string {
    return `${this.pathProvider.getApiUrl()}/v1/products/${value.id}/content`;
  }
}
