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
use crate::font::FONT;
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
                ImageSmoothingMode::NearestNeighbor,
                UVec2::new(8, 8 * 512),
                FONT.flatten(),
            )
            .unwrap();
        // let image_handle = graphics
        //     .create_image_from_file_path(
        //         Some(ImageFileFormat::PNG),
        //         ImageSmoothingMode::Linear,
        //         "cozette.png",
        //     )
        //     .unwrap();
        let spritesheet = Spritesheet::new(image_handle, 1, 512);
        self.spritesheets.push(spritesheet);
    }

    pub fn input(&mut self, viewport_size: UVec2, _mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
    }

    pub fn update(&mut self, current_frame: u64) {
        self.counter += 1;
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        let font = self.spritesheets.get(0).unwrap();
        let width = 16;
        let height = 16;
        for x in 0..font.width {
            for y in 0..font.height {
                let pos = Vec2::new(
                    (y / 20) as f32 * width as f32,
                    (y % 20) as f32 * height as f32,
                );
                font.draw_sprite(
                    &Rect::new(pos, pos + Vec2::new(width as f32, height as f32)),
                    x,
                    y,
                    graphics,
                )
            }
        }
    }
}
