# 🎨 Emoji Optimizer

A powerful desktop tool built with Tauri and vanilla JavaScript to help you design, analyze, and optimize pixel-perfect emojis. Break down your creations on a grid, count pixels, compare versions with layers, and perfect every detail.

---

## ✨ Key Features

* **Pixel-Perfect Canvas**: Load any image onto a 512x512 canvas. The tool automatically fits the image while preserving its aspect ratio, creating the perfect workspace for emoji design.
* **Toggleable Grid**: Overlay a 32x32 grid (16x16 pixel squares) to guide your design and analysis. Toggle visibility to see your work with or without the grid.
* **Counting Layers**: Add layers to count and annotate specific squares. This is perfect for analyzing color distribution or ensuring symmetry.
    * **Custom Colors**: Customize the background and font color for clear visual distinction between layers.
    * **Keyboard Navigation**: Quickly move between squares with `Enter` to count the next square or `Shift+Enter` to skip.
* **Optimization Zones**: Drag to create highlighted frames around specific areas. This feature helps you focus on sections that need refinement or compare color palettes between different parts of your emoji.
* **Project Persistence**: Save your entire workspace—the base image, all your layers, and annotations—into a single project file.
* **Export Your Work**: Export individual layers or the final composition to standard image formats like PNG.

## 🛠️ Tech Stack

* **Framework**: [Tauri](https://tauri.app/)
* **Frontend**: Vanilla JavaScript & [Alpine.js](https://alpinejs.dev/) for reactivity
* **Styling**: [Tailwind CSS](https://tailwindcss.com/)
* **Build Tool**: [esbuild](https://esbuild.github.io/) (via Tauri)
* **Package Manager**: [pnpm](https://pnpm.io/)

## 🚀 Getting Started

Follow these instructions to get a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You must have the [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) installed, which includes the Rust toolchain and Node.js with `pnpm`.

### Installation & Running

1.  **Clone the repository:**
    ```sh
    git clone [https://github.com/your-username/emoji-optimizer.git](https://github.com/your-username/emoji-optimizer.git)
    cd emoji-optimizer
    ```

2.  **Install dependencies:**
    ```sh
    pnpm install
    ```

3.  **Run in development mode:**
    This command will launch the app in a development window with hot-reloading enabled.
    ```sh
    pnpm tauri dev
    ```

4.  **Build the application:**
    To create a production-ready, optimized executable for your platform, run:
    ```sh
    pnpm tauri build
    ```

## 🗺️ Roadmap

The goal is to evolve this tool from a powerful utility into a full-featured emoji design suite.

* [ ] **MVP**: All core features listed above are functional.
* [ ] **V1**: A fully polished, interactive UI/UX.
* [ ] **Future**:
    * Support for GIFs and simple video animations.
    * Advanced layer comparison tools (e.g., difference and overlay modes).
    * Full customization of all visual elements (grid, frames, etc.).

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/your-username/emoji-optimizer/issues).

## 📄 License

This project is licensed under the MIT License - see the `LICENSE.md` file for details.
