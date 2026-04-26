use macroquad::prelude::*;

use crate::assets::{paths, sprites};
use crate::background_map::BackgroundMap;
use crate::enemies;
use crate::equipment::Gun;
use crate::global_constants::{
    GUN_INITIAL_TIME_COUNT, GUN_PROJECTILE_SPEED, GUN_RATE_OF_FIRE, ITEM_SPAWN_FREQUENCY_SECONDS,
    PLAYER_SPEED,
};
use crate::items::ItemGenerator;
use crate::player::Player;
use crate::run_state::RunState;
use crate::settings::Settings;
use crate::user_interface;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RunEvent {
    None,
    WaveComplete,
    PlayerDied,
}

pub struct GameRun {
    bg_map: BackgroundMap,
    player: Player,
    player_gun: Gun,
    enemies_generator: enemies::Generator,
    item_generator: ItemGenerator,
    run_state: RunState,
}

impl GameRun {
    pub async fn initialize() -> Option<Self> {
        let bg_map = match BackgroundMap::initialize(paths::BACKGROUND_MAP).await {
            Some(bg_map) => bg_map,
            None => {
                info!("couldnt load background");
                return None;
            }
        };

        let player = Player::initialize(
            PLAYER_SPEED,
            Some(&[sprites::PLAYER_IDLE, sprites::PLAYER_RUN]),
        )
        .await;

        let player_gun = Gun::initialize(
            player.size.clone(),
            GUN_PROJECTILE_SPEED,
            GUN_RATE_OF_FIRE,
            GUN_INITIAL_TIME_COUNT,
            paths::GUN,
            paths::BULLET,
        )
        .await;

        Some(Self {
            bg_map,
            player,
            player_gun,
            enemies_generator: enemies::Generator::initialize().await,
            item_generator: ItemGenerator::new(),
            run_state: RunState::new(),
        })
    }

    pub fn update(&mut self, settings: &Settings) -> RunEvent {
        if self.run_state.update() {
            return RunEvent::WaveComplete;
        }

        self.player.update_pos(&mut self.bg_map);
        self.player_gun
            .update(&self.bg_map, &self.player, settings.aim_mode, false);

        self.item_generator.update(ITEM_SPAWN_FREQUENCY_SECONDS);
        let collected = self
            .item_generator
            .collect_for_player(&self.player, &self.bg_map);
        let material_count = collected.iter().map(|item| item.material_value).sum();
        self.run_state.add_materials(material_count);
        self.player.pickup_items(collected);

        for enemy in self.enemies_generator.current_enemies.iter_mut() {
            enemy.chase(&self.player, &self.bg_map);
            enemy.detect_collision(
                &mut self.player_gun.projectiles,
                &mut self.player,
                &self.bg_map,
            );
        }

        let material_drop_positions = self.enemies_generator.update(
            self.run_state.enemy_spawn_frequency(),
            self.run_state.enemy_spawn_count(),
        );
        self.item_generator
            .spawn_materials_at(material_drop_positions);

        if self.player.is_dead() {
            RunEvent::PlayerDied
        } else {
            RunEvent::None
        }
    }

    pub fn draw(&mut self, font: &Font, _settings: &Settings, paused: bool) {
        self.bg_map.draw();
        self.player.draw(paused);
        self.player_gun.draw(&self.bg_map);
        self.player_gun.draw_projectiles(&self.bg_map);
        self.item_generator.draw(&self.bg_map);
        for enemy in self.enemies_generator.current_enemies.iter_mut() {
            enemy.draw(&self.bg_map, paused);
        }

        user_interface::draw_health_bar(&self.player);
        user_interface::draw_kill_count(font, self.enemies_generator.kill_count);
        user_interface::draw_run_stats(font, &self.run_state);
        self.player.inventory.draw();
    }

    pub fn reset(&mut self) {
        self.enemies_generator.clear();
        self.player_gun.clear();
        self.player.restart();
        self.item_generator.clear();
        self.run_state.reset();
    }

    pub fn enter_shop(&mut self) {
        self.enemies_generator.clear_current_enemies();
        self.player_gun.clear();
        self.item_generator.clear();
    }

    pub fn start_next_wave(&mut self) {
        self.run_state.start_next_wave();
    }

    pub fn draw_background(&mut self) {
        self.bg_map.draw();
    }

    pub fn run_state(&self) -> &RunState {
        &self.run_state
    }
}
