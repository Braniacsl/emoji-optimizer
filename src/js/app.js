import Alpine from 'alpinejs';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
// import { readBinaryFile, BaseDirectory } from '@tauri-apps/plugin-fs'; // FS plugin if needed directly in JS

document.addEventListener('alpine:init', () => {
  Alpine.data('imageEditor', () => ({
    imageUrl: null,
    yellowFilterApplied: false,
    errorMessage: '',

    async openImage() {
      this.errorMessage = ''; // Clear previous errors
      try {
        console.log("in");
        const selectedPath = await open({
          filters: [{ name: 'Images', extensions: ['png', 'jpeg', 'jpg', 'gif', 'webp'] }],
          multiple: false,
          directory: false,
          // title: "Select an Image" // Optional title
        });

        if (!selectedPath) {
          console.log("No file selected.");
          return;
        }
        const filePath = Array.isArray(selectedPath) ? selectedPath[0].path : selectedPath.path;

        // Now that we have the filePath, get its base64 representation via Rust command
        if (filePath) {
          const base64Image = await invoke('get_image_as_base64', { path: filePath });
          if (base64Image) {
            this.imageUrl = base64Image;
            this.yellowFilterApplied = false; // Reset filter on new image
          } else {
            this.errorMessage = "Could not load image data.";
            console.error("Received null or empty base64 image data.");
          }
        }
      } catch (error) {
        this.errorMessage = `Error opening image: ${error.toString()}`;
        console.error('Error opening image:', error);
        this.imageUrl = null; // Clear image on error
      } finally {
        if (this.errorMessage) {
          setTimeout(() => this.errorMessage = '', 5000); // Auto-hide error message
        }
      }
    },

    toggleYellowFilter() {
      if (this.imageUrl) {
        this.yellowFilterApplied = !this.yellowFilterApplied;
      } else {
        this.errorMessage = "Please open an image first to apply a filter.";
        setTimeout(() => this.errorMessage = '', 3000);
      }
    }
  }));
});

window.Alpine = Alpine;
Alpine.start();


