use std::f32::consts::PI;

use crate::{
    assets::SpriteSheetSpec, background_map::BackgroundMap, collision::Collision, custom::Point,
    equipment::Projectile, global_constants::ENEMY_CONTACT_DAMAGE, player::Player,
};
use animation::AnimatedSprite;
use macroquad::prelude::*;
use parry2d::{bounding_volume::Aabb, na::Point2};

// enemy ai
// move towards the player
// try attaking the player if the enemy has a weapon
// if enemy collides with player, reduce player's hp
// if enemy collides with projectile, reduce enemy's hp

#[derive(Clone)]
pub struct Enemy {
    pub pos: Point,
    pub size: Point,
    pub speed: f32, //pixel per frame
    pub hp: f32,
    pub hp_changed: bool, //this is used for hit animation, atm its used to just stop drawing when hit
    color: Color,
    hitbox_padding: f32,
    pub sprite_sheet: Option<AnimatedSprite>,
    pub texture: Vec<Texture2D>,
    flip_x: bool,
}

impl Enemy {
    pub async fn initialize(
        pos: Point,
        size: Point,
        speed: f32,
        hp: f32,
        color: Color,
        sprite_specs: Option<&[SpriteSheetSpec]>,
    ) -> Enemy {
        let mut enemy = Enemy {
            pos,
            size,
            speed,
            hp,
            hp_changed: false,
            color,
            hitbox_padding: 5.0,
            sprite_sheet: None,
            texture: vec![],
            flip_x: false,
        };

        match sprite_specs {
            Some(specs) => {
                for spec in specs.iter() {
                    let temp_texture = load_texture(spec.path).await;
                    match temp_texture {
                        Ok(a) => {
                            enemy.size.x = spec
                                .tile_width
                                .map_or(a.width() as f32 / spec.frames as f32, |width| {
                                    width as f32
                                });
                            enemy.size.y = spec
                                .tile_height
                                .map_or(a.height() as f32, |height| height as f32);
                            enemy.texture.push(a);
                        }
                        Err(_) => {
                            enemy.texture.clear();
                            return enemy;
                        }
                    }
                }
                enemy.sprite_sheet = Some(AnimatedSprite::new(
                    enemy.size.x as u32,
                    enemy.size.y as u32,
                    &[],
                    true,
                ));
            }
            None => todo!(),
        }

        return enemy;
    }

    pub fn detect_collision(
        &mut self,
        projectiles: &mut Vec<Projectile>,
        player: &mut Player,
        bg_map: &BackgroundMap,
    ) {
        //collision with projectiles
        projectiles.retain(|proj| {
            if (Collision {
                obj1: Aabb {
                    mins: Point2::new(
                        self.pos.x + self.hitbox_padding,
                        self.pos.y + self.hitbox_padding,
                    ),
                    maxs: Point2::new(
                        self.pos.x + self.size.x - self.hitbox_padding,
                        self.pos.y + self.size.y - self.hitbox_padding,
                    ),
                },
                obj2: Aabb {
                    mins: Point2::new(proj.pos.x, proj.pos.y),
                    maxs: Point2::new(proj.pos.x + proj.size.x, proj.pos.y + proj.size.y),
                },
            }
            .intersect())
            {
                self.hp = self.hp - proj.damage;
                self.hp_changed = true;
                return false;
            } else {
                return true;
            }
        });

        //collision with player
        if (Collision {
            obj1: Aabb {
                mins: Point2::new(
                    self.pos.x + self.hitbox_padding,
                    self.pos.y + self.hitbox_padding,
                ),
                maxs: Point2::new(
                    self.pos.x + self.size.x - self.hitbox_padding,
                    self.pos.y + self.size.y - self.hitbox_padding,
                ),
            },
            obj2: Aabb {
                mins: Point2::new(player.pos.x - bg_map.pos.x, player.pos.y - bg_map.pos.y),
                maxs: Point2::new(
                    player.pos.x - bg_map.pos.x + player.size.x,
                    player.pos.y - bg_map.pos.y + player.size.y,
                ),
            },
        }
        .intersect())
        {
            player.hp_reduction_cooldown_counter += get_frame_time();
            if player.hp_reduction_cooldown_counter >= player.hp_reduction_cooldown_value {
                player.hp = player.hp - ENEMY_CONTACT_DAMAGE;
                player.hp_reduction_cooldown_counter = 0.;
                player.hp_dropped = true;
            }

            //info!("collided with enemy, player hp: {}", player.hp);
        }
    }

    //simple chase algorithm (follows the player)
    pub fn chase(&mut self, player: &Player, bg_map: &BackgroundMap) {
        let pos_x = self.pos.x + bg_map.pos.x;
        let pos_y = self.pos.y + bg_map.pos.y;

        let mut theta = ((player.pos.y - pos_y) / (player.pos.x - pos_x)).atan();
        if player.pos.x - pos_x < 0.0 {
            theta = theta - PI;
            self.flip_x = true;
        } else {
            self.flip_x = false;
        }

        self.pos = Point {
            x: self.pos.x + self.speed * get_frame_time() * theta.cos(),
            y: self.pos.y + self.speed * get_frame_time() * theta.sin(),
        };
    }

    //todo draw simple rects when the texture is unavailable
    pub fn draw(&mut self, bg_map: &BackgroundMap, pause: bool) {
        if self.texture.len() > 0 {
            match &mut self.sprite_sheet {
                Some(a1) => {
                    let anim_index = 0;
                    a1.set_animation(anim_index);
                    if !self.hp_changed {
                        draw_texture_ex(
                            &self.texture[anim_index],
                            self.pos.x + bg_map.pos.x,
                            self.pos.y + bg_map.pos.y,
                            WHITE,
                            DrawTextureParams {
                                source: Some(a1.frame().source_rect),
                                dest_size: Some(a1.frame().dest_size),
                                rotation: 0.0,
                                flip_x: self.flip_x,
                                flip_y: false,
                                pivot: None,
                            },
                        );
                        if !pause {
                            a1.update();
                        }
                    } else {
                        self.hp_changed = false;
                    }
                }
                None => {
                    draw_rectangle(
                        self.pos.x + bg_map.pos.x,
                        self.pos.y + bg_map.pos.y,
                        self.size.x,
                        self.size.y,
                        self.color,
                    );
                }
            }
        } else {
            draw_rectangle(
                self.pos.x + bg_map.pos.x,
                self.pos.y + bg_map.pos.y,
                self.size.x,
                self.size.y,
                self.color,
            );
        }
    }
}
