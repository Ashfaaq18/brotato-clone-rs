use macroquad::prelude::*;
use macroquad::ui::root_ui;

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
use crate::user_interface;
use crate::user_interface::{get_menu_skin, UiSkins};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    MainMenu,
    Playing,
    Shop,
    Paused,
    GameOver,
}

pub struct App {
    bg_map: BackgroundMap,
    player: Player,
    player_gun: Gun,
    main_menu: user_interface::MainMenu,
    pause_menu: user_interface::PauseMenu,
    gameover_menu: user_interface::GameOverMenu,
    wave_shop_menu: user_interface::WaveShopMenu,
    enemies_generator: enemies::Generator,
    item_generator: ItemGenerator,
    run_state: RunState,
    font: Font,
    _main_menu_ui: macroquad::ui::Skin,
    ui_skins: UiSkins,
    screen: Screen,
    quit_requested: bool,
}

impl App {
    pub async fn initialize() -> Option<Self> {
        info!("Initializing modules");

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

        let font = user_interface::initialize_font().await;
        let main_menu_ui = get_menu_skin(&font).await;
        let ui_skins = user_interface::UiSkins::new(&font);

        root_ui().push_skin(&main_menu_ui);

        Some(Self {
            bg_map,
            player,
            player_gun,
            main_menu: user_interface::MainMenu::initialize(),
            pause_menu: user_interface::PauseMenu::initialize(),
            gameover_menu: user_interface::GameOverMenu::initialize(),
            wave_shop_menu: user_interface::WaveShopMenu::initialize(),
            enemies_generator: enemies::Generator::initialize().await,
            item_generator: ItemGenerator::new(),
            run_state: RunState::new(),
            font,
            _main_menu_ui: main_menu_ui,
            ui_skins,
            screen: Screen::MainMenu,
            quit_requested: false,
        })
    }

    pub fn update(&mut self) -> bool {
        if self.quit_requested {
            return false;
        }

        match self.screen {
            Screen::MainMenu => self.update_main_menu(),
            Screen::Playing => self.update_gameplay(),
            Screen::Shop => {}
            Screen::Paused => {}
            Screen::GameOver => {}
        }

        !self.quit_requested
    }

    pub fn draw(&mut self) {
        match self.screen {
            Screen::MainMenu => self.draw_main_menu(),
            Screen::Playing | Screen::Shop | Screen::Paused | Screen::GameOver => self.draw_run(),
        }
    }

    pub fn reset_run(&mut self) {
        self.enemies_generator.clear();
        self.player_gun.clear();
        self.player.restart();
        self.item_generator.clear();
        self.run_state.reset();
        self.gameover_menu.draw = false;
        self.pause_menu.mainmenu = false;
        self.pause_menu.resume = true;
        self.pause_menu.restart = false;
        self.gameover_menu.mainmenu = false;
        self.gameover_menu.restart = false;
        self.wave_shop_menu.reset();
    }

    fn update_main_menu(&mut self) {
        if self.main_menu.play {
            self.screen = Screen::Playing;
        }

        if self.main_menu.quit {
            self.quit_requested = true;
        }
    }

    fn update_gameplay(&mut self) {
        self.pause_menu.update();
        if self.run_state.update() {
            self.enter_wave_shop();
            return;
        }

        self.player.update_pos(&mut self.bg_map);
        self.player_gun.update_pos(&self.bg_map, &self.player);

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
            self.gameover_menu.draw = true;
        }

        if !self.pause_menu.resume {
            self.screen = Screen::Paused;
        } else if self.gameover_menu.draw {
            self.screen = Screen::GameOver;
        }
    }

    fn draw_main_menu(&mut self) {
        self.bg_map.draw();
        self.main_menu.draw(&self.ui_skins);

        if self.main_menu.play {
            self.screen = Screen::Playing;
        }

        if self.main_menu.quit {
            self.quit_requested = true;
        }
    }

    fn draw_run(&mut self) {
        let paused = self.screen == Screen::Shop
            || self.screen == Screen::Paused
            || self.screen == Screen::GameOver;

        self.bg_map.draw();
        self.player.draw(paused);
        self.player_gun.draw_gun(
            &self.bg_map,
            paused,
            self.main_menu.options.keyboard_to_shoot,
        );
        self.player_gun.draw_projectiles(&self.bg_map);
        self.item_generator.draw(&self.bg_map);
        for enemy in self.enemies_generator.current_enemies.iter_mut() {
            enemy.draw(&self.bg_map, paused);
        }

        user_interface::draw_health_bar(&self.player);
        user_interface::draw_kill_count(&self.font, self.enemies_generator.kill_count);
        user_interface::draw_run_stats(&self.font, &self.run_state);
        self.player.inventory.draw();

        match self.screen {
            Screen::Shop => self.draw_wave_shop_menu(),
            Screen::Paused => self.draw_pause_menu(),
            Screen::GameOver => self.draw_gameover_menu(),
            _ => {}
        }
    }

    fn enter_wave_shop(&mut self) {
        self.enemies_generator.clear_current_enemies();
        self.player_gun.clear();
        self.item_generator.clear();
        self.wave_shop_menu.reset();
        self.screen = Screen::Shop;
    }

    fn draw_wave_shop_menu(&mut self) {
        user_interface::draw_opaque_background();
        self.wave_shop_menu.draw(&self.run_state);

        if self.wave_shop_menu.continue_run {
            self.run_state.start_next_wave();
            self.wave_shop_menu.reset();
            self.screen = Screen::Playing;
        }

        if self.wave_shop_menu.mainmenu {
            self.reset_run();
            self.main_menu.play = false;
            self.screen = Screen::MainMenu;
        }

        if self.wave_shop_menu.quit {
            self.quit_requested = true;
        }
    }

    fn draw_pause_menu(&mut self) {
        user_interface::draw_opaque_background();
        self.pause_menu.draw();

        if self.pause_menu.mainmenu || self.pause_menu.restart {
            let return_to_main_menu = self.pause_menu.mainmenu;
            self.reset_run();
            self.main_menu.play = !return_to_main_menu;
            self.screen = if return_to_main_menu {
                Screen::MainMenu
            } else {
                Screen::Playing
            };
        } else if self.pause_menu.resume {
            self.screen = Screen::Playing;
        }

        if self.pause_menu.quit {
            self.quit_requested = true;
        }
    }

    fn draw_gameover_menu(&mut self) {
        user_interface::draw_opaque_background();
        self.gameover_menu.draw();

        if self.gameover_menu.mainmenu || self.gameover_menu.restart {
            let return_to_main_menu = self.gameover_menu.mainmenu;
            self.reset_run();
            self.main_menu.play = !return_to_main_menu;
            self.screen = if return_to_main_menu {
                Screen::MainMenu
            } else {
                Screen::Playing
            };
        }

        if self.gameover_menu.quit {
            self.quit_requested = true;
        }
    }
}
