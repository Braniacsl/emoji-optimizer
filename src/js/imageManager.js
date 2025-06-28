import { listen } from '@tauri-apps/api/event';

export const imageManager = {
  imageUrl: '',
  errorMessage: '',
  activeLayer: 0,

  async init() {
    await listen('image-updated', (event) => {
      this.imageUrl = event.payload;
    });
  },
};
