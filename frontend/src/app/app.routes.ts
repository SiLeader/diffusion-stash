import {Routes} from '@angular/router';
import {ModelGallery} from './ui/pages/model-gallery/model-gallery';
import {UploadModel} from './ui/pages/upload-model/upload-model';
import {ModelDetail} from './ui/pages/model-detail/model-detail';
import {UploadImage} from './ui/pages/upload-image/upload-image';
import {ImageGallery} from './ui/pages/image-gallery/image-gallery';

export const routes: Routes = [
  {path: '', redirectTo: 'models', pathMatch: 'full'},
  {path: 'models', component: ModelGallery},
  {path: 'upload/model', component: UploadModel},
  {path: 'upload/image', component: UploadImage},
  {path: 'models/:id', component: ModelDetail},
  {path: 'models/:id/gallery', component: ImageGallery},
];
