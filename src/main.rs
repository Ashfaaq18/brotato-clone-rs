mod app;
mod assets;
mod background_map;
mod collision;
mod combat;
mod custom;
mod enemies;
mod enemy;
mod game_run;
mod global_constants;
mod input;
mod inventory;
mod items;
mod player;
mod run_state;
mod settings;
mod user_interface;

use app::App;
pub use background_map::BackgroundMap;
use core::time;
use global_constants::FPS;
pub use global_constants::{GAME_TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};
use macroquad::prelude::*;
use std::{thread::sleep, time::SystemTime};

fn conf() -> Conf {
    Conf {
        window_title: GAME_TITLE.to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut app = match App::initialize().await {
        Some(app) => app,
        None => return,
    };

    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        if !app.update() {
            return;
        }

        app.draw();

        fps_control(now);
        next_frame().await
    }
}

fn fps_control(now: SystemTime) {
    match now.elapsed() {
        Ok(elapsed) => {
            let fps_duration = time::Duration::from_secs_f32(1.0 / FPS);
            if elapsed < fps_duration {
                let sleep_duration = fps_duration - elapsed;
                if sleep_duration > time::Duration::from_micros(0) {
                    sleep(sleep_duration);
                }
            }
        }
        Err(e) => {
            error!("Error: {e:?}");
        }
    }
}
