import { TestBed } from '@angular/core/testing';

import { ModelCategoryProvider } from './model-category-provider';

describe('ModelCategoryProvider', () => {
  let service: ModelCategoryProvider;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ModelCategoryProvider);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
