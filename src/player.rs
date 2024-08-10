use animation::{AnimatedSprite, Animation};
use macroquad::{prelude::*, texture};
use crate::{custom::{Direction, Point}, input::Movement, BackgroundMap, WINDOW_HEIGHT, WINDOW_WIDTH};

const WIDTH: f32 = 40.0;
const HEIGHT: f32 = 40.0;
const WORLDBITSIZE: f32 = 32.0/2.0;
pub struct Player {
    pub pos: Point,
    pub mov: Movement,
    pub size: Point,
    sprite_sheet: Option<AnimatedSprite>,
    pub texture: Vec<Texture2D>,
}

impl Player {
    pub async fn initialize(speed: f32, texture_paths: Option<&[&str]>) -> Player {
        let mut player = Player {
            pos: Point {
                x: WINDOW_WIDTH/2.0 - WIDTH/2.0,
                y: WINDOW_HEIGHT/2.0 - HEIGHT/2.0,
            },
            mov: Movement {
                speed,
                dir: Direction {
                    point: Point {
                        x: 0.0,
                        y: 0.0,
                    }
                }
            },
            size: Point {
                x: WIDTH,
                y: HEIGHT,
            },
            sprite_sheet: None,
            texture: vec![],
        };
        
        match texture_paths {
            Some(texture_path_some) => {
                let mut tile_width = 0;
                let mut tile_height = 0;
                let mut animations: Vec<Animation> = vec![];
                for texture_path in texture_path_some.iter() {
                    //the following scope has hardcoded values, try fix this
                    let idle_png = "assets\\topdown_shooter_assets\\sPlayerIdle_strip4.png";
                    let run_png = "assets\\topdown_shooter_assets\\sPlayerRun_strip7.png";
                    if texture_path.contains(idle_png) {
                        let temp_texture = load_texture(idle_png).await;
                        match temp_texture {
                            Ok(a) => { player.texture.push(a)},
                            Err(_) =>{
                                player.texture.clear();
                                return player;
                            },
                        }
                        animations.push(
                            Animation {
                                name: idle_png.to_string(),
                                row: 0,
                                frames: 4,
                                fps: 12,
                            }
                        );
                        tile_width = WIDTH as u32;
                        tile_height = HEIGHT as u32;
                    } else if texture_path.contains(run_png) {
                        let temp_texture = load_texture(run_png).await;
                        match temp_texture {
                            Ok(a) => { player.texture.push(a)},
                            Err(_) =>{
                                player.texture.clear();
                                return player;
                            },
                        }
                        animations.push(
                            Animation {
                                name: run_png.to_string(),
                                row: 0,
                                frames: 7,
                                fps: 12,
                            }
                        );
                        tile_width = WIDTH as u32;
                        tile_height = HEIGHT as u32;
                    }
                }
                player.sprite_sheet = Some(AnimatedSprite::new(
                    tile_width,
                    tile_height,
                    &animations,
                    true,
                ));
            },
            None => {},
        }
        player
    }

    pub fn update_pos(&mut self, map: &mut BackgroundMap, player_vel: &Point) {
        let pos =  self.pos.clone() + player_vel.clone();

        let screen_half_size_x = WINDOW_WIDTH/2.0 - self.size.x / 2.0;
        let screen_half_size_y = WINDOW_HEIGHT/2.0 - self.size.y / 2.0;
        
        //camera
        if pos.x < screen_half_size_x + map.pos.x {
            if pos.x < WORLDBITSIZE {
                //boundary
                self.pos.x = WORLDBITSIZE;
            }else {
                self.pos.x += player_vel.x;
            }
            map.pos.x = 0.0;
        } else if pos.x > screen_half_size_x + map.pos.x + map.background_img.width() - WINDOW_WIDTH {
            if pos.x > WINDOW_WIDTH - self.size.x - WORLDBITSIZE {
                //boundary
                self.pos.x = WINDOW_WIDTH - self.size.x - WORLDBITSIZE;
            } else {
                self.pos.x += player_vel.x;
            }
        } else {
            map.pos.x -= player_vel.x;
            self.pos.x = screen_half_size_x;
        }

        if pos.y < screen_half_size_y + map.pos.y  {
            if pos.y < WORLDBITSIZE { 
                //boundary
                self.pos.y = WORLDBITSIZE;
            } else {
                self.pos.y += player_vel.y;
            }
            map.pos.y = 0.0;
        } else if pos.y > screen_half_size_y + map.pos.y + map.background_img.height() - WINDOW_HEIGHT {
            if pos.y > WINDOW_HEIGHT- self.size.y - WORLDBITSIZE {
                //boundary
                self.pos.y = WINDOW_HEIGHT - self.size.y - WORLDBITSIZE;
            } else {
                self.pos.y += player_vel.y;
            }
        } else {
            map.pos.y -= player_vel.y;
            self.pos.y = screen_half_size_y;
        }
    }

    //todo draw simple rects when the texture is unavailable
    pub fn draw(&mut self, player_vel: &Point) {
        if self.texture.len() > 0 {
            let anim_index;
            if player_vel.x == 0.0 && player_vel.y == 0.0 {
                anim_index = 0;
            } else {
                anim_index = 1;
            }
            match &mut self.sprite_sheet {
                Some(a1) => {
                    a1.set_animation(anim_index);
                    draw_texture_ex(
                        &self.texture[anim_index], 
                        self.pos.x, 
                        self.pos.y, 
                        WHITE, 
                        DrawTextureParams{
                            source: Some(a1.frame().source_rect),
                            dest_size: Some(a1.frame().dest_size),
                            rotation: 0.0,
                            flip_x: match player_vel.x { a => { if a < 0.0 {true} else {false} } },
                            flip_y: false,
                            pivot: None,
                        });
                    a1.update();
                },
                None => {
                    draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, ORANGE);
                },
            }
        } else {
            draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, ORANGE);
        }
        
        
    }
}