import {Pipe, PipeTransform} from '@angular/core';
import {Model} from '../../apis/data/model';
import {environment} from '../../../environments/environment';

@Pipe({
  name: 'thumbnail'
})
export class ThumbnailPipe implements PipeTransform {

  transform(value: Model): string {
    return `${environment.apiUrl}/v1/models/${value.id}/thumbnail`
  }
}
