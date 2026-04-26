use crate::{
    custom,
    global_constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
};
use macroquad::prelude::*;

pub struct BackgroundMap {
    pub background_img: Texture2D,
    pub pos: custom::Point,
}

impl BackgroundMap {
    pub async fn initialize(path: &str) -> Option<BackgroundMap> {
        let texture = load_texture(path).await;
        return if texture.is_err() {
            error!("couldn't load the map");
            None
        } else {
            info!("map loaded!");
            let img = texture.unwrap();
            let width = img.width();
            let height = img.height();
            Option::from(BackgroundMap {
                background_img: img,
                pos: custom::Point {
                    x: -1.0 * width / 2.0 + WINDOW_WIDTH / 2.0,
                    y: -1.0 * height / 2.0 + WINDOW_HEIGHT / 2.0,
                },
            })
        };
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.background_img,
            self.pos.x,
            self.pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    self.background_img.width(),
                    self.background_img.height(),
                )),
                ..Default::default()
            },
        );
    }

    pub fn world_to_screen(&self, world_pos: custom::Point) -> custom::Point {
        world_pos + self.pos
    }

    pub fn screen_to_world(&self, screen_pos: custom::Point) -> custom::Point {
        screen_pos - self.pos
    }

    pub fn width(&self) -> f32 {
        self.background_img.width()
    }

    pub fn height(&self) -> f32 {
        self.background_img.height()
    }

    pub fn contains_world_point(&self, world_pos: custom::Point) -> bool {
        world_pos.x >= 0.0
            && world_pos.y >= 0.0
            && world_pos.x <= self.width()
            && world_pos.y <= self.height()
    }
}
