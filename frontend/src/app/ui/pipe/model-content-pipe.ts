import {Pipe, PipeTransform} from '@angular/core';
import {Model} from '../../apis/data/model';
import {environment} from '../../../environments/environment';

@Pipe({
  name: 'modelContent'
})
export class ModelContentPipe implements PipeTransform {
  transform(value: Model): string {
    return `${environment.apiUrl}/v1/models/${value.id}/content`;
  }
}
