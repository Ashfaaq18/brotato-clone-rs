use crate::{
    custom::Point, global_constants::GUN_PROJECTILE_DAMAGE, input, player::Player, BackgroundMap,
};
use macroquad::prelude::*;
use std::f32::consts::PI;

pub struct Projectile {
    pub pos: Point,
    pub size: Point,
    pub damage: f32,
    params: DrawRectangleParams,
}

pub struct Gun {
    size: Point,
    pos: Point,
    pub texture: Option<Texture2D>,
    projectile_speed: f32,
    pub projectiles: Vec<Projectile>,
    pub projectile_texture: Option<Texture2D>,
    rate_of_fire: f32,
    time_count: f32,
}

impl Gun {
    pub async fn initialize(
        size: Point,
        projectile_speed: f32,
        rate_of_fire: f32,
        time_count: f32,
        path: &str,
        path_projectile_texture: &str,
    ) -> Gun {
        let texture = load_texture(path).await;
        let projectile_texture = load_texture(path_projectile_texture).await;

        let mut gun = Gun {
            size,
            pos: Point { x: 0.0, y: 0.0 },
            projectile_speed,
            rate_of_fire,
            time_count,
            projectiles: vec![],
            texture: None,
            projectile_texture: None,
        };

        match projectile_texture {
            Ok(a) => {
                gun.projectile_texture = Some(a);
            }
            Err(_) => {}
        }

        match texture {
            Ok(a) => {
                gun.texture = Some(a);
                gun
            }
            Err(_) => gun,
        }
    }

    pub fn update_pos(&mut self, bg_map: &BackgroundMap, player: &Player) {
        //update gun position
        self.pos.x = player.pos.x;
        self.pos.y = player.pos.y;

        //update projectile position (remove if it goes out of map bounds)
        self.projectiles.retain_mut(|proj| {
            proj.pos.x =
                proj.pos.x + proj.params.rotation.cos() * self.projectile_speed * get_frame_time();
            proj.pos.y =
                proj.pos.y + proj.params.rotation.sin() * self.projectile_speed * get_frame_time();
            bg_map.contains_world_point(proj.pos)
        });
    }

    pub fn draw_gun(&mut self, bg_map: &BackgroundMap, pause: bool, keyb_to_shoot: bool) {
        //draw gun
        self.size.x = self.size.x * 1.2;
        self.size.y = self.size.y / 4.0;

        match &self.texture {
            Some(a) => {
                self.size.x = a.width();
                self.size.y = a.height();
            }
            None => {}
        }

        let mut x: f32 = self.pos.x + (self.size.x / 2.0);
        let mut y = self.pos.y + self.size.y / 2.0;

        let mut theta = 0.0;
        let mut params = DrawRectangleParams {
            offset: Vec2 { x: 0.0, y: 0.5 },
            rotation: 0.0,
            color: DARKBROWN,
        };

        if !keyb_to_shoot {
            let point_to_pos = input::get_cursor_pos();
            let dis =
                f32::sqrt(f32::powf(point_to_pos.x - x, 2.0) + f32::powf(point_to_pos.y - y, 2.0));
            theta = ((point_to_pos.y - y) / dis).acos();

            if point_to_pos.x < x {
                theta = PI / 2.0 + theta;
            } else {
                theta = PI / 2.0 - theta;
            }

            x = x + theta.cos();
            y = y + theta.sin() + self.size.y / 2.0;

            params = DrawRectangleParams {
                offset: Vec2 { x: 0.0, y: 0.5 },
                rotation: theta,
                color: DARKBROWN,
            };

            if is_mouse_button_down(macroquad::input::MouseButton::Left)
                && (self.time_count > 1.0 / self.rate_of_fire)
                && !pause
            {
                let projectile_screen_pos = Point {
                    x: x + self.size.x * theta.cos(),
                    y: y + self.size.y * theta.sin(),
                };
                let projectile_world_pos = bg_map.screen_to_world(projectile_screen_pos);
                self.projectiles.push(Projectile {
                    pos: projectile_world_pos,
                    size: Point { x: 0.0, y: 0.0 },
                    damage: GUN_PROJECTILE_DAMAGE,
                    params: params.clone(),
                });
                self.time_count = 0.0;
                //info!("mouse clicked, timecount: {}", self.time_count);
            }
        } else {
        }

        self.time_count += get_frame_time();

        match &self.texture {
            Some(a) => {
                draw_texture_ex(
                    &a,
                    x,
                    y - a.height() / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(a.width(), a.height())),
                        source: None,
                        rotation: theta,
                        flip_x: false,
                        flip_y: false,
                        pivot: Some(Vec2 { x: x, y: y }),
                    },
                );
            }
            None => {
                draw_rectangle_ex(x, y, self.size.x, self.size.y, params);
            }
        }
    }

    pub fn draw_projectiles(&mut self, bg_map: &BackgroundMap) {
        for proj in self.projectiles.iter() {
            let screen_pos = bg_map.world_to_screen(proj.pos);
            match &self.projectile_texture {
                Some(a) => {
                    draw_texture_ex(
                        &a,
                        screen_pos.x - a.width() / 2.0,
                        screen_pos.y - a.height() / 2.0,
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
                        proj.size.x,
                        proj.size.y,
                        proj.params.clone(),
                    );
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.projectiles.clear();
    }
}
