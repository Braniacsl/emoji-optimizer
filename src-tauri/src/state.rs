use image::{Rgba, RgbaImage};
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::Mutex as TokioMutex;

pub struct AppState {
    pub image_manager: TokioMutex<ImageManager>,
}

#[derive(Debug)]
pub struct ImageManager {
    pub width: u32,
    pub height: u32,
    pub layers: Vec<Arc<StdMutex<Layer>>>,
    pub next_layer_id: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layer {
    pub id: u32,
    pub name: String,
    pub buffer: RgbaImage,
    pub position: Point,
    pub width: u32,
    pub height: u32,
    pub is_visible: bool,
    pub grid: GridOptions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridOptions {
    pub is_visible: bool,
    pub line_color: Rgba<u8>,
    pub line_thickness: u32,
    pub cell_count: u32,
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
