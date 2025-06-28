use image::{imageops, Rgba, RgbaImage};
use log::info;
use std::sync::{Arc, Mutex as StdMutex};

use crate::state::{GridOptions, ImageManager, Layer, Point};
use anyhow::{anyhow, Result};

impl ImageManager {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            layers: Vec::new(),
            next_layer_id: 0,
        }
    }

    pub fn render(&self) -> Result<RgbaImage, String> {
        info!("[ImageManager] Rendering final image.");

        let mut canvas = RgbaImage::new(self.width, self.height);

        for layer in self.layers.iter() {
            let layer = layer.lock().unwrap();
            if layer.is_visible {
                imageops::overlay(
                    &mut canvas,
                    &layer.buffer,
                    layer.position.x as i64,
                    layer.position.y as i64,
                );
            }
        }

        info!("[ImageManager] Render complete.");

        Ok(canvas)
    }

    pub fn save_layer(&self, layer_id: u32, path: &str) -> Result<(), image::ImageError> {
        unimplemented!()
    }

    // Returns the id of the new layer
    pub fn add_layer_from_image(
        &mut self,
        image_buffer: RgbaImage,
        name: String,
        position: Point,
    ) -> Result<u32> {
        let curr_id = self.get_layer_id();

        //Resizes image if larger or smaller than ImageManager
        let resized_buffer = imageops::resize(
            &image_buffer,
            self.width,
            self.height,
            imageops::FilterType::Lanczos3,
        );

        self.layers.push(Arc::new(StdMutex::new(Layer {
            id: curr_id,
            name,
            buffer: resized_buffer,
            position,
            width: self.width,
            height: self.height,
            is_visible: true,
            grid: GridOptions::new(self.width, self.height),
        })));

        Ok(curr_id)
    }

    fn get_layer_id(&mut self) -> u32 {
        self.next_layer_id += 1;
        self.next_layer_id
    }

    pub fn add_layer_grid(&mut self, name: &str, position: Point) -> u32 {
        unimplemented!()
    }

    pub fn get_layer(&self, layer_id: u32) -> Option<&Layer> {
        unimplemented!()
    }

    pub fn get_layer_mut(&mut self, layer_id: u32) -> Option<&mut Layer> {
        unimplemented!()
    }

    pub fn remove_layer(&mut self, layer_id: u32) -> anyhow::Result<()> {
        unimplemented!()
    }

    pub fn set_layer_visibility(&mut self, layer_id: u32) -> anyhow::Result<()> {
        unimplemented!()
    }
}
