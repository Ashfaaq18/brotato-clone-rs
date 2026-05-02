use parry2d::bounding_volume::Aabb;

use crate::{
    collision::{aabb_from_pos_size, Collision},
    custom::Point,
    global_constants::{
        GUN_INITIAL_TIME_COUNT, GUN_PROJECTILE_DAMAGE, GUN_PROJECTILE_SPEED, GUN_RATE_OF_FIRE,
    },
    input::{self, AimMode},
    player::Player,
    BackgroundMap,
};
use macroquad::prelude::*;

pub struct Combat {
    weapon: Weapon,
    projectiles: Projectiles,
}

impl Combat {
    pub async fn initialize(player_size: Point, weapon_path: &str, projectile_path: &str) -> Self {
        Self {
            weapon: Weapon::initialize(
                player_size,
                GUN_RATE_OF_FIRE,
                GUN_INITIAL_TIME_COUNT,
                weapon_path,
            )
            .await,
            projectiles: Projectiles::initialize(GUN_PROJECTILE_SPEED, projectile_path).await,
        }
    }

    pub fn update(&mut self, bg_map: &BackgroundMap, player: &Player, aim_mode: AimMode) {
        self.weapon.update_pos(player);
        self.projectiles.update(bg_map);

        if let Some(projectile) = self.weapon.try_fire(bg_map, aim_mode) {
            self.projectiles.push(projectile);
        }
    }

    pub fn draw(&self, bg_map: &BackgroundMap) {
        self.weapon.draw();
        self.projectiles.draw(bg_map);
    }

    pub fn projectiles_mut(&mut self) -> &mut Projectiles {
        &mut self.projectiles
    }

    pub fn upgrade_fire_rate(&mut self, delta: f32) {
        self.weapon.rate_of_fire += delta;
    }

    pub fn upgrade_damage(&mut self, delta: f32) {
        self.weapon.base_damage += delta;
    }

    pub fn clear(&mut self) {
        self.projectiles.clear();
    }
}

pub struct Weapon {
    size: Point,
    pos: Point,
    texture: Option<Texture2D>,
    rate_of_fire: f32,
    time_count: f32,
    aim_direction: Point,
    base_damage: f32,
}

impl Weapon {
    pub async fn initialize(size: Point, rate_of_fire: f32, time_count: f32, path: &str) -> Weapon {
        let texture = load_texture(path).await;

        let mut weapon = Weapon {
            size,
            pos: Point { x: 0.0, y: 0.0 },
            rate_of_fire,
            time_count,
            texture: None,
            aim_direction: Point { x: 1.0, y: 0.0 },
            base_damage: GUN_PROJECTILE_DAMAGE,
        };

        match texture {
            Ok(a) => {
                weapon.texture = Some(a);
                weapon
            }
            Err(_) => weapon,
        }
    }

    pub fn update_pos(&mut self, player: &Player) {
        self.pos.x = player.pos.x;
        self.pos.y = player.pos.y;
    }

    pub fn try_fire(&mut self, bg_map: &BackgroundMap, aim_mode: AimMode) -> Option<Projectile> {
        let (x, y, theta, params) = self.draw_state(aim_mode);

        let projectile = if self.time_count > 1.0 / self.rate_of_fire {
            self.time_count = 0.0;
            let (draw_width, draw_height) = self.draw_size();
            let projectile_screen_pos = Point {
                x: x + draw_width * theta.cos(),
                y: y + draw_height * theta.sin(),
            };

            Some(Projectile {
                pos: bg_map.screen_to_world(projectile_screen_pos),
                size: Point { x: 0.0, y: 0.0 },
                damage: self.base_damage,
                params,
            })
        } else {
            None
        };

        self.time_count += get_frame_time();
        projectile
    }

    pub fn draw(&self) {
        let (draw_width, draw_height) = self.draw_size();
        let theta = self.aim_direction.y.atan2(self.aim_direction.x);
        let x = self.pos.x + (draw_width / 2.0) + theta.cos();
        let y = self.pos.y + draw_height / 2.0 + theta.sin() + draw_height / 2.0;

        let params = DrawRectangleParams {
            offset: Vec2 { x: 0.0, y: 0.5 },
            rotation: theta,
            color: DARKBROWN,
        };

        match &self.texture {
            Some(texture) => {
                draw_texture_ex(
                    texture,
                    x,
                    y - texture.height() / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(texture.width(), texture.height())),
                        source: None,
                        rotation: theta,
                        flip_x: false,
                        flip_y: false,
                        pivot: Some(Vec2 { x, y }),
                    },
                );
            }
            None => {
                draw_rectangle_ex(x, y, draw_width, draw_height, params);
            }
        }
    }

    fn draw_state(&mut self, aim_mode: AimMode) -> (f32, f32, f32, DrawRectangleParams) {
        let (draw_width, draw_height) = self.draw_size();
        let mut x = self.pos.x + (draw_width / 2.0);
        let mut y = self.pos.y + draw_height / 2.0;
        let aim_direction = match aim_mode {
            AimMode::Mouse => input::get_cursor_pos() - Point { x, y },
            AimMode::Keyboard => input::get_keyboard_aim_direction().unwrap_or(self.aim_direction),
        };

        if aim_direction.x != 0.0 || aim_direction.y != 0.0 {
            self.aim_direction = aim_direction;
        }
        let theta = self.aim_direction.y.atan2(self.aim_direction.x);

        x += theta.cos();
        y += theta.sin() + draw_height / 2.0;

        let params = DrawRectangleParams {
            offset: Vec2 { x: 0.0, y: 0.5 },
            rotation: theta,
            color: DARKBROWN,
        };

        (x, y, theta, params)
    }

    fn draw_size(&self) -> (f32, f32) {
        match &self.texture {
            Some(texture) => (texture.width(), texture.height()),
            None => (self.size.x * 1.2, self.size.y / 4.0),
        }
    }
}

pub struct Projectile {
    pub pos: Point,
    pub size: Point,
    pub damage: f32,
    params: DrawRectangleParams,
}

pub struct Projectiles {
    items: Vec<Projectile>,
    speed: f32,
    texture: Option<Texture2D>,
}

impl Projectiles {
    pub async fn initialize(speed: f32, texture_path: &str) -> Self {
        Self {
            items: Vec::new(),
            speed,
            texture: load_texture(texture_path).await.ok(),
        }
    }

    pub fn push(&mut self, projectile: Projectile) {
        self.items.push(projectile);
    }

    pub fn update(&mut self, bg_map: &BackgroundMap) {
        self.items.retain_mut(|projectile| {
            projectile.pos.x += projectile.params.rotation.cos() * self.speed * get_frame_time();
            projectile.pos.y += projectile.params.rotation.sin() * self.speed * get_frame_time();
            bg_map.contains_world_point(projectile.pos)
        });
    }

    pub fn damage_colliding_with(&mut self, target: Aabb) -> f32 {
        let mut damage = 0.0;
        self.items.retain(|projectile| {
            let hit = (Collision {
                obj1: target.clone(),
                obj2: aabb_from_pos_size(projectile.pos, projectile.size),
            })
            .intersect();

            if hit {
                damage += projectile.damage;
                false
            } else {
                true
            }
        });
        damage
    }

    pub fn draw(&self, bg_map: &BackgroundMap) {
        for projectile in self.items.iter() {
            let screen_pos = bg_map.world_to_screen(projectile.pos);
            match &self.texture {
                Some(texture) => {
                    draw_texture_ex(
                        texture,
                        screen_pos.x - texture.width() / 2.0,
                        screen_pos.y - texture.height() / 2.0,
                        WHITE,
                        DrawTextureParams {
                            ..Default::default()
                        },
                    );
                }
                None => {
                    draw_rectangle_ex(
                        screen_pos.x,
                        screen_pos.y,
                        projectile.size.x,
                        projectile.size.y,
                        projectile.params.clone(),
                    );
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }
}
