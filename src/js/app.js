import Alpine from 'alpinejs';
import { imageViewer } from './imageViewer';
import { fileDropdown } from './fileDropdown';
import { toolbox } from './toolbox';
import { imageManager } from './imageManager';

document.addEventListener('alpine:init', () => {
  Alpine.store('imageManager', imageManager)
  Alpine.data('imageViewer', imageViewer);
  Alpine.data('fileDropdown', fileDropdown);
  Alpine.data('toolbox', toolbox);
});

document.addEventListener('DOMContentLoaded', () => {
  Alpine.store('imageManager').init();
});

window.Alpine = Alpine;
Alpine.start();


