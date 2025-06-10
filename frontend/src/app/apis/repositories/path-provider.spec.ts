import { TestBed } from '@angular/core/testing';

import { PathProvider } from './path-provider';

describe('PathProvider', () => {
  let service: PathProvider;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(PathProvider);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
