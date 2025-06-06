import {TestBed} from '@angular/core/testing';

import {ModelRepositoryImpl} from './model-repository';

describe('ModelRepositoryImpl', () => {
  let service: ModelRepositoryImpl;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(ModelRepositoryImpl);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
