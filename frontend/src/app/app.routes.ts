import {Routes} from '@angular/router';
import {Landing} from './ui/pages/landing/landing';
import {UploadModel} from './ui/pages/upload-model/upload-model';
import {ModelDetail} from './ui/pages/model-detail/model-detail';
import {UploadImage} from './ui/pages/upload-image/upload-image';

export const routes: Routes = [
  {path: '', component: Landing},
  {path: 'upload/model', component: UploadModel},
  {path: 'upload/image', component: UploadImage},
  {path: 'models/:id', component: ModelDetail},
];
