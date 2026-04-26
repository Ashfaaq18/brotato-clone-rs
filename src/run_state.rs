use macroquad::time::get_frame_time;

use crate::global_constants::{
    ENEMY_SPAWN_COUNT, ENEMY_SPAWN_FREQUENCY_SECONDS, WAVE_DURATION_SECONDS,
    WAVE_SPAWN_COUNT_BONUS_INTERVAL, WAVE_SPAWN_FREQUENCY_STEP_SECONDS,
};

pub struct RunState {
    pub wave: i32,
    pub wave_elapsed: f32,
    pub materials: i32,
}

impl RunState {
    pub fn new() -> Self {
        Self {
            wave: 1,
            wave_elapsed: 0.0,
            materials: 0,
        }
    }

    pub fn update(&mut self) {
        self.wave_elapsed += get_frame_time();
        if self.wave_elapsed >= WAVE_DURATION_SECONDS {
            self.wave += 1;
            self.wave_elapsed = 0.0;
        }
    }

    pub fn add_materials(&mut self, amount: i32) {
        self.materials += amount;
    }

    pub fn enemy_spawn_frequency(&self) -> f32 {
        let scaled_frequency = ENEMY_SPAWN_FREQUENCY_SECONDS
            - ((self.wave - 1) as f32 * WAVE_SPAWN_FREQUENCY_STEP_SECONDS);
        scaled_frequency.max(1.0)
    }

    pub fn enemy_spawn_count(&self) -> i32 {
        ENEMY_SPAWN_COUNT + ((self.wave - 1) / WAVE_SPAWN_COUNT_BONUS_INTERVAL)
    }

    pub fn wave_time_remaining(&self) -> f32 {
        (WAVE_DURATION_SECONDS - self.wave_elapsed).max(0.0)
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }
}
