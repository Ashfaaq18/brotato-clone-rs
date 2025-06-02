mod input;
mod custom;
mod background_map;
mod player;
mod enemy;
mod enemies;
mod equipment;
mod collision;
mod user_interface;
mod global_constants;
mod inventory;
mod items;

use background_map::BackgroundMap;
use equipment::Gun;
use global_constants::{GAME_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT, FPS};
use macroquad::ui::root_ui;
use player::Player;
use user_interface::get_menu_skin;
use items::ItemGenerator;
use core::time;
use std::{thread::sleep, time::SystemTime};
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf{
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
    info!("Initializing modules");

    //map
    let bg_map = BackgroundMap::initialize("assets/background_map.png").await;
    if bg_map.is_none() {
        info!("couldnt load background");
        return;
    }
    let mut bg_map = bg_map.unwrap();
    
    // player
    let mut player = Player::initialize(
        100.0, 
        Some(&[
            "assets\\topdown_shooter_assets\\sPlayerIdle_strip4.png",
            "assets\\topdown_shooter_assets\\sPlayerRun_strip7.png"
            ])).await;

    let mut player_gun = Gun::initialize(
        player.size.clone(),
        300.0,
        3.0,
        0.0,
        "assets\\topdown_shooter_assets\\sGun.png",
        "assets\\topdown_shooter_assets\\sBullet.png").await;
    
    let mut main_menu = user_interface::MainMenu::initialize();
    let mut pause_menu = user_interface::PauseMenu::initialize();
    let mut gameover_menu = user_interface::GameOverMenu::initialize();

    let mut enemies_generator = enemies::Generator::initialize().await;
    let mut item_generator = ItemGenerator::new();

    let font: Font = user_interface::initialize_font().await;
    let main_menu_ui = get_menu_skin(&font).await;
    let ui_skins = user_interface::UiSkins::new(&font);

    root_ui().push_skin(&main_menu_ui);
    
    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        // main menu
        if !main_menu.play {
            bg_map.draw();
            main_menu.draw(&ui_skins);
            if main_menu.quit { return; }
        } 
        // game
        else { 

            //run the logic here
            if pause_menu.resume && !gameover_menu.draw {
                pause_menu.update();

                //update
                player.update_pos(&mut bg_map);
                player_gun.update_pos(&bg_map, &player);

                item_generator.update(5.0);
                let collected = item_generator.collect_for_player(&player, &bg_map);
                player.pickup_items(collected);
                
                for enemy in enemies_generator.current_enemies.iter_mut() {
                    enemy.chase(&player, &bg_map);
                    enemy.detect_collision(&mut player_gun.projectiles, &mut player, &bg_map);
                }

                //every 5 seconds generate 2 enemies
                enemies_generator.update(4.0, 2);
                if player.is_dead() {
                    gameover_menu.draw = true;
                }
            }
            
            // draw
            bg_map.draw();
            player.draw( !pause_menu.resume|| gameover_menu.draw);
            player_gun.draw_gun(&bg_map, !pause_menu.resume|| gameover_menu.draw, main_menu.options.keybToShoot);
            player_gun.draw_projectiles(&bg_map);
            item_generator.draw(&bg_map);
            for enemy in enemies_generator.current_enemies.iter_mut() {
                enemy.draw(&bg_map, !pause_menu.resume || gameover_menu.draw);
            }
            
            
            //to do: refactor the ui flow below
            // ------------------- UI stuff ---------------------------------//
            
            user_interface::draw_health_bar(&player);
            user_interface::draw_kill_count(&font, enemies_generator.kill_count);
            player.inventory.draw();
            
            //pause menu
            if !pause_menu.resume {
                user_interface::draw_opaque_background();
                pause_menu.draw();

                if pause_menu.mainmenu || pause_menu.restart { 
                    //clear all internal states before going back to main menu or restarting
                    enemies_generator.clear();
                    player_gun.clear();
                    player.restart();
                    main_menu.play = !pause_menu.mainmenu; //set the rhs to default after use
                    pause_menu.mainmenu = false; //revert the state to default
                    pause_menu.resume = true;
                    pause_menu.restart = false;
                }

                if pause_menu.quit {return;}
            } else if gameover_menu.draw {
                user_interface::draw_opaque_background();
                gameover_menu.draw();
                
                if gameover_menu.mainmenu || gameover_menu.restart {
                    //clear all internal states before going back to main menu
                    enemies_generator.clear();
                    player_gun.clear();
                    player.restart();
                    gameover_menu.draw = false;
                    main_menu.play = !gameover_menu.mainmenu; //set the rhs to default after use
                    gameover_menu.mainmenu = false; //revert the state to default
                    gameover_menu.restart = false;
                }
                
                if gameover_menu.quit {return;}
            } 
            
        }

        fps_control(now);
        next_frame().await
    }
}

fn fps_control(now: SystemTime) {
    match now.elapsed() {
        Ok(elapsed) => {
            let fps_duration = time::Duration::from_secs_f32(1.0/FPS);
            if elapsed < fps_duration {
                let sleep_duration = fps_duration - elapsed;
                if sleep_duration > time::Duration::from_micros(0) {
                    sleep(fps_duration - elapsed);
                }
            }
        }
        Err(e) => {
            error!("Error: {e:?}");
        }
    }
}