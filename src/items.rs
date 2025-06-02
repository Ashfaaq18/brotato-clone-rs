use macroquad::prelude::*;
use crate::{custom::Point, global_constants::{WINDOW_WIDTH, WINDOW_HEIGHT}, BackgroundMap, player::Player};
use ::rand::prelude::*;

#[derive(Clone)]
pub struct Item {
    pub pos: Point,
    pub size: Point,
    pub color: Color,
}

pub struct ItemGenerator {
    pub items: Vec<Item>,
    counter: f32,
}

impl ItemGenerator {
    pub fn new() -> Self {
        Self { items: Vec::new(), counter: 0.0 }
    }

    fn spawn_random(&mut self) {
        let mut rng = thread_rng();
        let size = Point { x: 20.0, y: 20.0 };
        let pos: Point = Point {
            x: rng.gen_range(20.0..(WINDOW_WIDTH - 20.0)),
            y: rng.gen_range(20.0..(WINDOW_HEIGHT - 20.0))
        };
        let color = Color { r: 0.55, g: 0.16, b: 0.16, a: 1.  };
        self.items.push(Item { pos, size, color });
    }

    pub fn update(&mut self, frequency: f32) {
        self.counter += get_frame_time();
        if self.counter > frequency {
            self.spawn_random();
            self.counter = 0.0;
        }
    }

    pub fn draw(&self, bg_map: &BackgroundMap) {
        for it in &self.items {
            draw_rectangle(it.pos.x + bg_map.pos.x, it.pos.y + bg_map.pos.y, it.size.x, it.size.y, it.color);
        }
    }

    pub fn collect_for_player(&mut self, player: &Player, bg_map: &BackgroundMap) -> Vec<Item> {
        let mut collected = Vec::new();
        self.items.retain(|it| {
            let px = player.pos.x - bg_map.pos.x;
            let py = player.pos.y - bg_map.pos.y;
            let colliding = px < it.pos.x + it.size.x &&
                px + player.size.x > it.pos.x &&
                py < it.pos.y + it.size.y &&
                py + player.size.y > it.pos.y;
            if colliding {
                collected.push(it.clone());
                false
            } else {
                true
            }
        });
        collected
    }
}