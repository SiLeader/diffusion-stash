import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ModelInfoTable } from './model-info-table';

describe('ModelInfoTable', () => {
  let component: ModelInfoTable;
  let fixture: ComponentFixture<ModelInfoTable>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ModelInfoTable]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ModelInfoTable);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
