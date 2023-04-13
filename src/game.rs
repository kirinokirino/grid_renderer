use speedy2d::color::Color;
use speedy2d::image::ImageFileFormat;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode;
use speedy2d::Graphics2D;

use glam::{UVec2, Vec2};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
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
        // let image_handle = graphics
        //     .create_image_from_file_path(
        //         Some(ImageFileFormat::PNG),
        //         ImageSmoothingMode::Linear,
        //         "assets/robot.png",
        //     )
        //     .unwrap();
        // self.images.push(image_handle);
    }

    pub fn input(&mut self, viewport_size: UVec2, _mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
    }

    pub fn update(&mut self, current_frame: u64) {
        self.counter += 1;
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {}
}
