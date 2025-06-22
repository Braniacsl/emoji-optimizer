use image::{Rgba, RgbaImage};
use tauri::async_runtime::Mutex;

pub struct AppState {
    pub image_manager: Mutex<ImageManager>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageManager {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Layer>,
    pub next_layer_id: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub id: u32,
    pub name: String,
    pub buffer: RgbaImage,
    pub position: Point,
    pub is_visible: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridOptions {
    pub line_color: Rgba<u8>,
    pub line_thickness: u32,
    pub cell_width: u32,
    pub cell_height: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextOptions<'a> {
    pub text: &'a str,
    pub font_data: &'a [u8],
    pub font_size: f32,
    pub font_color: Rgba<u8>,
}
