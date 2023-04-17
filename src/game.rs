use glam_rect::Rect;
use speedy2d::color::Color;
use speedy2d::image::ImageDataType;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode;
use speedy2d::Graphics2D;

use glam::{UVec2, Vec2};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
use crate::font::vga8;
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
                ImageDataType::RGBA,
                ImageSmoothingMode::NearestNeighbor,
                UVec2::new(8, 16 * 256),
                &vga8(),
            )
            .unwrap();
        self.spritesheets
            .push(Spritesheet::new(image_handle, 1, 256));
    }

    pub fn input(&mut self, viewport_size: UVec2, _mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
    }

    pub fn update(&mut self, current_frame: u64) {
        self.counter += 1;
    }

    pub fn draw_char(
        &self,
        ch: char,
        position: Vec2,
        color: Color,
        bg_color: Color,
        graphics: &mut Graphics2D,
    ) {
        let vga8 = self.spritesheets.get(0).unwrap();
        let width = self.config.grid_width;
        let height = self.config.grid_height;
        let rect = Rect::new(position, position + Vec2::new(width as f32, height as f32));
        graphics.draw_rectangle(rect.clone(), bg_color);
        vga8.draw_sprite_with_color(&rect, 0, ch.into(), color, graphics);
    }

    pub fn draw_string(
        &self,
        str: &str,
        position: Vec2,
        color: Color,
        bg_color: Color,
        graphics: &mut Graphics2D,
    ) {
        let vga8 = self.spritesheets.get(0).unwrap();
        let width = self.config.grid_width;
        let height = self.config.grid_height;
        for (i, char_idx) in str.chars().map(|ch| ch as u8).enumerate() {
            let pos = position + Vec2::new((u32::try_from(i) * width) as f32, 0.0);
            let rect = Rect::new(pos, pos + Vec2::new(width as f32, height as f32));
            graphics.draw_rectangle(rect.clone(), bg_color);
            vga8.draw_sprite_with_color(&rect, 0, char_idx.into(), color, graphics);
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        self.draw_string(
            "1234567890',.ygcrl/=aoeuidhtns-;qjkxbmwvz?:;",
            Vec2::new(16.0, 32.0),
            Color::RED,
            Color::BLUE,
            graphics,
        );
    }
}
