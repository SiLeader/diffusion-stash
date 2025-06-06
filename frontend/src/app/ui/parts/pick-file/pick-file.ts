import {Component, ElementRef, forwardRef, HostListener, input, ViewChild} from '@angular/core';
import {MatButton, MatIconButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {ControlValueAccessor, NG_VALUE_ACCESSOR} from '@angular/forms';
import {MatFormField} from '@angular/material/input';

@Component({
  selector: 'app-pick-file',
  imports: [
    MatIcon,
    MatIconButton,
    MatFormField,
  ],
  providers: [
    {
      provide: NG_VALUE_ACCESSOR,
      useExisting: forwardRef(() => PickFile),
      multi: true
    }
  ],
  templateUrl: './pick-file.html',
  styleUrl: './pick-file.css'
})
export class PickFile implements ControlValueAccessor {
  @ViewChild('fileInput') fileInput!: ElementRef<HTMLInputElement>;

  fileName: string | null = null;
  isDragging = false;

  onChange: (file: File | null) => void = () => {
  };
  onTouched: () => void = () => {
  };

  writeValue(value: any): void {
  }

  registerOnChange(fn: any): void {
    this.onChange = fn;
  }

  registerOnTouched(fn: any): void {
    this.onTouched = fn;
  }

  @HostListener('dragover', ['$event'])
  onDragOver(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.isDragging = true;
  }

  @HostListener('dragleave', ['$event'])
  onDragLeave(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.isDragging = false;
  }

  @HostListener('drop', ['$event'])
  onDrop(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.isDragging = false;

    const files = event.dataTransfer?.files;
    if (files && files.length > 0) {
      this.handleFile(files[0]);
    }
  }

  onContainerClick(): void {
    this.fileInput.nativeElement.click();
  }

  onFileSelected(event: Event): void {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      this.handleFile(input.files[0]);
    }
  }

  private handleFile(file: File): void {
    this.fileName = file.name;
    this.onChange(file);
    this.onTouched();
  }

  clearFile(event: MouseEvent): void {
    event.stopPropagation();
    this.fileName = null;
    this.fileInput.nativeElement.value = '';
    this.onChange(null);
    this.onTouched();
  }
}
