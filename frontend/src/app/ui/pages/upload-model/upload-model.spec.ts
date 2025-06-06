import { ComponentFixture, TestBed } from '@angular/core/testing';

import { UploadModel } from './upload-model';

describe('UploadModel', () => {
  let component: UploadModel;
  let fixture: ComponentFixture<UploadModel>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [UploadModel]
    })
    .compileComponents();

    fixture = TestBed.createComponent(UploadModel);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
