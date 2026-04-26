use macroquad::input::{is_key_down, mouse_position, KeyCode};
use macroquad::time::get_frame_time;
use crate::custom::{Direction, Point};

pub fn get_cursor_pos() -> Point {
    let pos = mouse_position();
    return Point {
        x: pos.0,
        y: pos.1
    }
}

pub struct Movement{
    pub speed: f32, // pixel per frame
    pub dir: Direction,
}

impl Movement {
    // 1 frame => 1 pixel * speed constant
    // (pixel * speed) per frame
    pub fn register_keyboard_press(&mut self) -> Point {

        if is_key_down(KeyCode::A) { //left
            self.dir.point.x = -1.0;
        } else if is_key_down(KeyCode::D) { //right
            self.dir.point.x = 1.0;
        } else {
            self.dir.point.x = 0.0;
        }

        if is_key_down(KeyCode::W) { //up
            self.dir.point.y = -1.0;
        } else if is_key_down(KeyCode::S) { //down
            self.dir.point.y = 1.0;
        } else {
            self.dir.point.y = 0.0;
        }        

        return self.dir.point.clone() * self.speed * get_frame_time();
    }   
}
