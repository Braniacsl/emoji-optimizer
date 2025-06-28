export const imageViewer = () => ({
  zoom: 1,

  handleWheel(event) {
    const scaleAmount = -event.deltaY > 0 ? 1.1 : 1 / 1.1;
    this.applyZoom(scaleAmount);
  },

  zoomIn() {
    this.applyZoom(1.2);
  },

  zoomOut() {
    this.applyZoom(1 / 1.2);
  },

  applyZoom(scaleAmount) {
    const newZoom = this.zoom * scaleAmount;

    if (scaleAmount < 1) {
      this.zoom = Math.max(0.1, newZoom);
      return;
    }

    if (!this.$refs.image) return;

    const viewport = this.$refs.imageDisplay;
    const image = this.$refs.image;

    const futureWidth = image.clientWidth * scaleAmount;
    const futureHeight = image.clientHeight * scaleAmount;

    if (futureWidth < viewport.clientWidth && futureHeight < viewport.clientHeight) {
      this.zoom = newZoom;
    }
  },

  resetZoom() {
    this.zoom = 1;
  }
});
