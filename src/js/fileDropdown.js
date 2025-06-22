import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

export const fileDropdown = () => ({
  open: false,

  toggle() {
    if (this.open) {
      return this.close();
    }

    this.$refs.button.focus();

    this.open = true;
  },
  close(focusAfter) {
    if (!this.open) return;

    this.open = false;

    focusAfter && focusAfter.focus();
  },
  async open_image() {
    const filePath = await open({
      multiple: false,
      directory: false,
      filters: [{
        name: 'Image',
        extensions: ['png', 'jpg', 'jpeg']
      }]
    });

    if (filePath === null) {
      alert("Failed to select file");
      return;
    }

    this.close(this.$refs.button);

    invoke('open_image', { path: filePath });
  },
})
