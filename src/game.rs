use glam_rect::Rect;
use speedy2d::color::Color;
use speedy2d::image::ImageDataType;
use speedy2d::image::ImageFileFormat;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode;
use speedy2d::Graphics2D;

use glam::{UVec2, Vec2};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
use crate::font::VGA8;
use crate::spritesheet::Spritesheet;

pub struct Game {
    config: Config,
    images: Vec<ImageHandle>,
    spritesheets: Vec<Spritesheet>,
    counter: usize,

    viewport_size: UVec2,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        Self {
            config,
            images: Vec::new(),
            spritesheets: Vec::new(),

            counter: 0,
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        let image_handle = graphics
            .create_image_from_raw_pixels(
                ImageDataType::RGB,
                ImageSmoothingMode::Linear,
                UVec2::new(8, 16 * 256),
                &to_rgb_bytes(&VGA8),
            )
            .unwrap();
        // let image_handle = graphics
        //     .create_image_from_file_path(
        //         Some(ImageFileFormat::PNG),
        //         ImageSmoothingMode::Linear,
        //         "cozette.png",
        //     )
        //     .unwrap();
        let spritesheet = Spritesheet::new(image_handle, 1, 256);
        self.spritesheets.push(spritesheet);
    }

    pub fn input(&mut self, viewport_size: UVec2, _mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
    }

    pub fn update(&mut self, current_frame: u64) {
        self.counter += 1;
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        let vga8 = self.spritesheets.get(0).unwrap();
        let scale = 1;
        let padding = 0;
        let width = 8 * scale;
        let height = 16 * scale;
        for x in 0..vga8.width {
            for y in 0..vga8.height {
                let pos = Vec2::new(
                    (y % 16) as f32 * (width + padding) as f32,
                    (y / 16) as f32 * (height + padding) as f32,
                );
                vga8.draw_sprite(
                    &Rect::new(pos, pos + Vec2::new(width as f32, height as f32)),
                    x,
                    y,
                    graphics,
                )
            }
        }
    }
}

pub fn to_rgb_bytes(vga8: &[[u8; 16]; 256]) -> Vec<u8> {
    let mut expanded: Vec<u8> = Vec::with_capacity(3 * 8 * 16 * 256);
    for character in vga8 {
        for line in character {
            for i in 0..8 {
                let mask = 1 << i;
                let bit = (mask & line) > 0;
                match bit {
                    true => {
                        expanded.push(255);
                        expanded.push(255);
                        expanded.push(255);
                    }
                    false => {
                        expanded.push(0);
                        expanded.push(0);
                        expanded.push(0);
                    }
                }
            }
        }
    }
    expanded
}
