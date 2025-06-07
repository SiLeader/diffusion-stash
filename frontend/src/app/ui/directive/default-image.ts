import {Directive, HostBinding, HostListener, Input, input} from '@angular/core';

@Directive({
  selector: 'img[defaultSrc]'
})
export class DefaultImage {
  defaultSrc = input.required<string>();

  @HostBinding('attr.src')
  @Input()
  src!: string;

  @HostListener('error')
  replaceToDefault() {
    this.src = this.defaultSrc();
  }
}
