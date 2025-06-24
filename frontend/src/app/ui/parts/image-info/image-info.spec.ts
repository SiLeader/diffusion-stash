import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ImageInfo } from './image-info';

describe('ImageInfo', () => {
  let component: ImageInfo;
  let fixture: ComponentFixture<ImageInfo>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ImageInfo]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ImageInfo);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
