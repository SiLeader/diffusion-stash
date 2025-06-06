import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PickFile } from './pick-file';

describe('PickFile', () => {
  let component: PickFile;
  let fixture: ComponentFixture<PickFile>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [PickFile]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PickFile);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
