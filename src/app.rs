use macroquad::prelude::*;
use macroquad::ui::root_ui;

use crate::game_run::{GameRun, RunEvent};
use crate::settings::Settings;
use crate::user_interface;
use crate::user_interface::{get_menu_skin, get_options_skin};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    MainMenu,
    Playing,
    Shop,
    Paused,
    GameOver,
}

pub struct App {
    game_run: GameRun,
    settings: Settings,
    main_menu: user_interface::MainMenu,
    pause_menu: user_interface::PauseMenu,
    gameover_menu: user_interface::GameOverMenu,
    wave_shop_menu: user_interface::WaveShopMenu,
    font: Font,
    _main_menu_ui: macroquad::ui::Skin,
    options_ui: macroquad::ui::Skin,
    screen: Screen,
    quit_requested: bool,
}

impl App {
    pub async fn initialize() -> Option<Self> {
        info!("Initializing modules");

        let game_run = GameRun::initialize().await?;

        let font = user_interface::initialize_font().await;
        let main_menu_ui = get_menu_skin(&font).await;
        let options_ui = get_options_skin(&font).await;
        root_ui().push_skin(&main_menu_ui);

        Some(Self {
            game_run,
            settings: Settings::new(),
            main_menu: user_interface::MainMenu::initialize(),
            pause_menu: user_interface::PauseMenu::initialize(),
            gameover_menu: user_interface::GameOverMenu::initialize(),
            wave_shop_menu: user_interface::WaveShopMenu::initialize(),
            font,
            _main_menu_ui: main_menu_ui,
            options_ui,
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
        self.game_run.reset();
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

        match self.game_run.update(&self.settings) {
            RunEvent::None => {}
            RunEvent::WaveComplete => {
                self.enter_wave_shop();
                return;
            }
            RunEvent::PlayerDied => {
                self.gameover_menu.draw = true;
            }
        }

        if !self.pause_menu.resume {
            self.screen = Screen::Paused;
        } else if self.gameover_menu.draw {
            self.screen = Screen::GameOver;
        }
    }

    fn draw_main_menu(&mut self) {
        self.game_run.draw_background();
        self.main_menu.draw(&self.options_ui, &mut self.settings);

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

        self.game_run.draw(&self.font, &self.settings, paused);

        match self.screen {
            Screen::Shop => self.draw_wave_shop_menu(),
            Screen::Paused => self.draw_pause_menu(),
            Screen::GameOver => self.draw_gameover_menu(),
            _ => {}
        }
    }

    fn enter_wave_shop(&mut self) {
        self.game_run.enter_shop();
        self.wave_shop_menu.reset();
        self.screen = Screen::Shop;
    }

    fn draw_wave_shop_menu(&mut self) {
        user_interface::draw_opaque_background();
        self.wave_shop_menu.draw(self.game_run.run_state());

        if self.wave_shop_menu.continue_run {
            self.game_run.start_next_wave();
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
