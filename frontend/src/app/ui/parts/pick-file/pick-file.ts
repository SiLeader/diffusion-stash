import {Component, ElementRef, forwardRef, ViewChild} from '@angular/core';
import {MatIconButton} from '@angular/material/button';
import {MatIcon} from '@angular/material/icon';
import {ControlValueAccessor, NG_VALUE_ACCESSOR} from '@angular/forms';
import {MatCard, MatCardContent} from '@angular/material/card';

@Component({
  selector: 'app-pick-file',
  imports: [
    MatIcon,
    MatIconButton,
    MatCard,
    MatCardContent,
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
  imagePreviewUrl: string | ArrayBuffer | null = null; // 画像プレビュー用のURLを保持するプロパティを追加
  isDragging = false;

  onChange: (file: File | null) => void = () => {
  };
  onTouched: () => void = () => {
  };

  writeValue(value: any): void {
    // Note: This is intentionally left blank.
  }

  registerOnChange(fn: any): void {
    this.onChange = fn;
  }

  registerOnTouched(fn: any): void {
    this.onTouched = fn;
  }

  onDragOver(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.isDragging = true;
  }

  onDragLeave(event: DragEvent): void {
    event.preventDefault();
    event.stopPropagation();
    this.isDragging = false;
  }

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
    // プレビュー画像やファイル情報がクリックされてもダイアログを開く
    if (!this.fileName) {
      this.fileInput.nativeElement.click();
    }
  }

  // ファイル選択ボタンを明示的にクリックするためのメソッド
  openFilePicker(): void {
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

    // ファイルが画像かどうかをMIMEタイプで判定
    if (file.type.startsWith('image/')) {
      const reader = new FileReader();
      reader.onload = () => {
        this.imagePreviewUrl = reader.result; // 読み込み結果をプレビューURLに設定
      };
      reader.readAsDataURL(file); // ファイルをData URLとして読み込む
    } else {
      this.imagePreviewUrl = null; // 画像でなければプレビューはnull
    }

    this.onChange(file);
    this.onTouched();
  }

  clearFile(event: MouseEvent): void {
    event.stopPropagation();
    this.fileName = null;
    this.imagePreviewUrl = null; // プレビューURLもクリア
    this.fileInput.nativeElement.value = '';
    this.onChange(null);
    this.onTouched();
  }
}
