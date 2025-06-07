import {Pipe, PipeTransform} from '@angular/core';
import {environment} from '../../../environments/environment';
import {Product} from '../../apis/data/product';

@Pipe({
  name: 'productContent'
})
export class ProductContentPipe implements PipeTransform {
  transform(value: Product): string {
    return `${environment.apiUrl}/v1/products/${value.id}/content`;
  }
}
