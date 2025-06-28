import { invoke } from '@tauri-apps/api/core';

export const toolbox = () => ({
  async toggleGrid() {
    await invoke('toggle_grid', { activeLayer: Alpine.store('imageManager').activeLayer });
  },
}); 
