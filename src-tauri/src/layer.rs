use image::{imageops, RgbaImage};
use imageproc::drawing::draw_line_segment_mut;
use log::info;

use crate::{
    constants,
    state::{GridOptions, Layer},
};

impl Layer {
    pub fn render(&mut self) {
        info!("[Layer] Rendering layer.");

        let mut canvas = RgbaImage::new(self.width, self.height);

        imageops::overlay(
            &mut canvas,
            &self.buffer,
            self.position.x as i64,
            self.position.y as i64,
        );

        if self.grid.is_visible {
            self.draw_grid(&mut canvas);
        }

        self.buffer = canvas;

        info!("[Layer] Finished rendering layer.")
    }

    fn draw_grid(&mut self, canvas: &mut RgbaImage) {
        info!("[Layer] Drawing grid");

        if self.grid.line_thickness == 0 {
            return;
        }

        for x in (0..self.grid.cell_count).flat_map(|cell_idx| {
            let cell_start = cell_idx * self.grid.cell_width;
            let left_border = cell_start..(cell_start + self.grid.line_thickness);
            let right_border = (cell_start + self.grid.cell_width - self.grid.line_thickness)
                ..(cell_start + self.grid.cell_width);
            left_border.chain(right_border)
        }) {
            if x <= self.width {
                draw_line_segment_mut(
                    canvas,
                    ((self.position.x + x) as f32, self.position.y as f32),
                    ((self.position.x + x) as f32, self.height as f32),
                    self.grid.line_color,
                );
            }
        }

        for y in (0..self.grid.cell_count).flat_map(|cell_idy| {
            let cell_start = cell_idy * self.grid.cell_height;
            let top_border = cell_start..(cell_start + self.grid.line_thickness);
            let bottom_border = (cell_start + self.grid.cell_height - self.grid.line_thickness)
                ..(cell_start + self.grid.cell_height);
            top_border.chain(bottom_border)
        }) {
            if y < self.height {
                draw_line_segment_mut(
                    canvas,
                    (self.position.x as f32, (self.position.y + y) as f32),
                    (self.height as f32, (self.position.y + y) as f32),
                    self.grid.line_color,
                );
            }
        }

        info!("[Layer] Finished drawing grid");
    }
}

impl GridOptions {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            is_visible: false,
            line_color: constants::LINE_COLOR,
            line_thickness: constants::LINE_THICKNESS,
            cell_count: constants::CELL_COUNT,
            cell_width: constants::CELL_WIDTH,
            cell_height: constants::CELL_HEIGHT,
        }
    }
}
