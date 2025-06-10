import {Pipe, PipeTransform} from '@angular/core';
import {Model} from '../../apis/data/model';
import {PathProvider} from '../../apis/repositories/path-provider';

@Pipe({
  name: 'modelContent'
})
export class ModelContentPipe implements PipeTransform {
  constructor(private pathProvider: PathProvider) {
  }

  transform(value: Model): string {
    return `${this.pathProvider.getApiUrl()}/v1/models/${value.id}/content`;
  }
}
