use std::f32::consts::PI;

use crate::{
    assets::SpriteSheetSpec,
    background_map::BackgroundMap,
    collision::{aabb_from_pos_size, padded_aabb_from_pos_size, Collision},
    custom::Point,
    equipment::Projectile,
    global_constants::ENEMY_CONTACT_DAMAGE,
    player::Player,
};
use animation::AnimatedSprite;
use macroquad::prelude::*;

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
                obj1: padded_aabb_from_pos_size(self.pos, self.size, self.hitbox_padding),
                obj2: aabb_from_pos_size(proj.pos, proj.size),
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
        let player_world_pos = player.world_pos(bg_map);
        if (Collision {
            obj1: padded_aabb_from_pos_size(self.pos, self.size, self.hitbox_padding),
            obj2: aabb_from_pos_size(player_world_pos, player.size),
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
        let player_world_pos = player.world_pos(bg_map);

        let mut theta =
            ((player_world_pos.y - self.pos.y) / (player_world_pos.x - self.pos.x)).atan();
        if player_world_pos.x - self.pos.x < 0.0 {
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
        let screen_pos = bg_map.world_to_screen(self.pos);
        if self.texture.len() > 0 {
            match &mut self.sprite_sheet {
                Some(a1) => {
                    let anim_index = 0;
                    a1.set_animation(anim_index);
                    if !self.hp_changed {
                        draw_texture_ex(
                            &self.texture[anim_index],
                            screen_pos.x,
                            screen_pos.y,
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
                        screen_pos.x,
                        screen_pos.y,
                        self.size.x,
                        self.size.y,
                        self.color,
                    );
                }
            }
        } else {
            draw_rectangle(
                screen_pos.x,
                screen_pos.y,
                self.size.x,
                self.size.y,
                self.color,
            );
        }
    }
}
