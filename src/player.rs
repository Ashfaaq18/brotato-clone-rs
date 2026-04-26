use crate::inventory::{Inventory, InventoryItem};
use crate::{
    assets::SpriteSheetSpec,
    custom::{Direction, Point},
    input::Movement,
    BackgroundMap, WINDOW_HEIGHT, WINDOW_WIDTH,
};
use animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;

const WIDTH: f32 = 40.0;
const HEIGHT: f32 = 40.0;
const WORLDBITSIZE: f32 = 32.0 / 2.0;
pub struct Player {
    pub pos: Point,
    pub mov: Movement,
    pub size: Point,
    pub hp: f32,
    pub hp_dropped: bool,
    pub hp_reduction_cooldown_counter: f32,
    pub hp_reduction_cooldown_value: f32,
    sprite_sheet: Option<AnimatedSprite>,
    pub texture: Vec<Texture2D>,
    pub inventory: Inventory,
}

impl Player {
    pub async fn initialize(speed: f32, sprite_specs: Option<&[SpriteSheetSpec]>) -> Player {
        let mut player = Player {
            pos: Point {
                x: WINDOW_WIDTH / 2.0 - WIDTH / 2.0,
                y: WINDOW_HEIGHT / 2.0 - HEIGHT / 2.0,
            },
            mov: Movement {
                speed,
                dir: Direction {
                    point: Point { x: 0.0, y: 0.0 },
                },
            },
            size: Point {
                x: WIDTH,
                y: HEIGHT,
            },
            hp: 100.,
            hp_dropped: false,
            hp_reduction_cooldown_counter: 0.,
            hp_reduction_cooldown_value: 0.5, //this is in seconds
            sprite_sheet: None,
            texture: vec![],
            inventory: Inventory::new(),
        };

        match sprite_specs {
            Some(specs) => {
                let mut tile_width = 0;
                let mut tile_height = 0;
                let mut animations: Vec<Animation> = vec![];
                for spec in specs.iter() {
                    let temp_texture = load_texture(spec.path).await;
                    match temp_texture {
                        Ok(texture) => player.texture.push(texture),
                        Err(_) => {
                            player.texture.clear();
                            return player;
                        }
                    }
                    animations.push(Animation {
                        name: spec.animation_name.to_string(),
                        row: spec.row,
                        frames: spec.frames,
                        fps: spec.fps,
                    });
                    tile_width = spec.tile_width.unwrap_or(WIDTH as u32);
                    tile_height = spec.tile_height.unwrap_or(HEIGHT as u32);
                }
                player.sprite_sheet = Some(AnimatedSprite::new(
                    tile_width,
                    tile_height,
                    &animations,
                    true,
                ));
            }
            None => {}
        }
        player
    }

    pub fn update_pos(&mut self, map: &mut BackgroundMap) {
        let player_vel = self.mov.register_keyboard_press();
        let pos = self.pos.clone() + player_vel.clone();

        let screen_half_size_x = WINDOW_WIDTH / 2.0 - self.size.x / 2.0;
        let screen_half_size_y = WINDOW_HEIGHT / 2.0 - self.size.y / 2.0;

        //camera
        if pos.x < screen_half_size_x + map.pos.x {
            if pos.x < WORLDBITSIZE {
                //boundary
                self.pos.x = WORLDBITSIZE;
            } else {
                self.pos.x += player_vel.x;
            }
            map.pos.x = 0.0;
        } else if pos.x > screen_half_size_x + map.pos.x + map.width() - WINDOW_WIDTH {
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

        if pos.y < screen_half_size_y + map.pos.y {
            if pos.y < WORLDBITSIZE {
                //boundary
                self.pos.y = WORLDBITSIZE;
            } else {
                self.pos.y += player_vel.y;
            }
            map.pos.y = 0.0;
        } else if pos.y > screen_half_size_y + map.pos.y + map.height() - WINDOW_HEIGHT {
            if pos.y > WINDOW_HEIGHT - self.size.y - WORLDBITSIZE {
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

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.
    }

    pub fn world_pos(&self, map: &BackgroundMap) -> Point {
        map.screen_to_world(self.pos)
    }

    //todo draw simple rects when the texture is unavailable
    pub fn draw(&mut self, pause: bool) {
        if self.texture.len() > 0 {
            let anim_index;
            if self.mov.dir.point.x == 0.0 && self.mov.dir.point.y == 0.0 {
                anim_index = 0;
            } else {
                anim_index = 1;
            }
            match &mut self.sprite_sheet {
                Some(a1) => {
                    a1.set_animation(anim_index);
                    if !self.hp_dropped {
                        draw_texture_ex(
                            &self.texture[anim_index],
                            self.pos.x,
                            self.pos.y,
                            WHITE,
                            DrawTextureParams {
                                source: Some(a1.frame().source_rect),
                                dest_size: Some(a1.frame().dest_size),
                                rotation: 0.0,
                                flip_x: match self.mov.dir.point.x {
                                    a => {
                                        if a < 0.0 {
                                            true
                                        } else {
                                            false
                                        }
                                    }
                                },
                                flip_y: false,
                                pivot: None,
                            },
                        );
                    } else {
                        self.hp_dropped = false;
                    }
                    if !pause {
                        a1.update();
                    }
                }
                None => {
                    draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, ORANGE);
                }
            }
        } else {
            draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, ORANGE);
        }
    }

    pub fn pickup_items(&mut self, items: Vec<crate::items::Item>) {
        for it in items {
            self.inventory.add(InventoryItem { color: it.color });
        }
    }

    pub fn restart(&mut self) {
        self.hp = 100.;
        self.pos = Point {
            x: WINDOW_WIDTH / 2.0 - WIDTH / 2.0,
            y: WINDOW_HEIGHT / 2.0 - HEIGHT / 2.0,
        };
        self.inventory.items.clear();
    }
}
