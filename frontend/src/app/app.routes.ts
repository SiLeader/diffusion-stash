import {Routes} from '@angular/router';
import {Landing} from './ui/pages/landing/landing';
import {UploadModel} from './ui/pages/upload-model/upload-model';
import {ModelDetail} from './ui/pages/model-detail/model-detail';

export const routes: Routes = [
  {path: '', component: Landing},
  {path: 'upload/model', component: UploadModel},
  {path: 'models/:id', component: ModelDetail},
];
