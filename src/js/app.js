import Alpine from 'alpinejs';
import { imageViewer } from './imageViewer';
import { fileDropdown } from './fileDropdown';

document.addEventListener('alpine:init', () => {
  Alpine.data('imageViewer', imageViewer);
  Alpine.data('fileDropdown', fileDropdown);
});

window.Alpine = Alpine;
Alpine.start();


