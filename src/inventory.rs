use macroquad::prelude::*;
use crate::global_constants::{WINDOW_WIDTH, WINDOW_HEIGHT};

#[derive(Clone)]
pub struct InventoryItem {
    pub color: Color,
}

pub struct Inventory {
    pub items: Vec<InventoryItem>,
}

impl Inventory {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add(&mut self, item: InventoryItem) {
        self.items.push(item);
    }

    pub fn draw(&self) {
        let size = 20.0;
        let mut x = 10.0;
        let y = WINDOW_HEIGHT - size - 5.0;
        for it in &self.items {
            draw_rectangle(x, y, size, size, it.color);
            x += size + 2.0;
            if x > WINDOW_WIDTH - 10.0 {
                x = 10.0;
            }
        }
    }
}