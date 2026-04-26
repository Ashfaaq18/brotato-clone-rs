use crate::{
    assets::sprites,
    custom::Point,
    enemy::Enemy,
    global_constants::{
        ENEMY_ANIMATION_FPS_MAX, ENEMY_ANIMATION_FPS_MIN, ENEMY_SPAWN_BORDER_PADDING,
        ENEMY_SPEED_MAX, ENEMY_SPEED_MIN, ENEMY_TEMPLATE_HP, ENEMY_TEMPLATE_SPEED, WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
};
use macroquad::{
    color::PINK,
    prelude::animation::{AnimatedSprite, Animation},
    time::get_frame_time,
};
use rand::prelude::*;

// enum EnemyType {
//     Small,  // small and fast
//     Medium, // default
//     Big,    // big and slow
// }

pub struct Generator {
    enemies_template: Vec<Enemy>, // store different types of enemies here and generate clones of it for use.
    pub current_enemies: Vec<Enemy>,
    counter: f32,        // time counter
    pub kill_count: i32, // enemy kill count
}

impl Generator {
    pub async fn initialize() -> Generator {
        return Generator {
            enemies_template: vec![
                //enemy type 1
                Enemy::initialize(
                    Point { x: 0.0, y: 0.0 },
                    Point { x: 0.0, y: 0.0 },
                    ENEMY_TEMPLATE_SPEED,
                    ENEMY_TEMPLATE_HP,
                    PINK,
                    Some(&[sprites::ENEMY_RUN]),
                )
                .await,
            ],
            current_enemies: vec![],
            counter: 0.,
            kill_count: 0,
        };
    }

    fn generate(&mut self, count: i32) {
        let enemy_type_1 = 0;
        //generate
        for _ in 0..count {
            let mut temp_x = WINDOW_WIDTH - self.enemies_template[0].size.x;
            let mut temp_y = WINDOW_HEIGHT - self.enemies_template[0].size.y;
            let border_pad = ENEMY_SPAWN_BORDER_PADDING;
            temp_x = rand::thread_rng().gen_range((border_pad)..(temp_x - border_pad)) as f32;
            temp_y = rand::thread_rng().gen_range((border_pad)..(temp_y - border_pad)) as f32;
            self.enemies_template[enemy_type_1].pos.x = temp_x;
            self.enemies_template[enemy_type_1].pos.y = temp_y;
            self.enemies_template[enemy_type_1].speed =
                rand::thread_rng().gen_range(ENEMY_SPEED_MIN..ENEMY_SPEED_MAX) as f32; //randomize the speed within a range to make the movement look random

            let mut animations: Vec<Animation> = vec![];
            animations.push(Animation {
                name: "run".to_string(),
                row: sprites::ENEMY_RUN.row,
                frames: sprites::ENEMY_RUN.frames,
                fps: rand::thread_rng().gen_range(ENEMY_ANIMATION_FPS_MIN..ENEMY_ANIMATION_FPS_MAX)
                    as u32, //this is estimated and set roughly by observing the frame transitions. randomize the animation fps to make the movement look random
            });
            self.enemies_template[enemy_type_1].sprite_sheet = Some(AnimatedSprite::new(
                self.enemies_template[enemy_type_1].size.x as u32,
                self.enemies_template[enemy_type_1].size.y as u32,
                &animations,
                true,
            ));

            self.current_enemies
                .push(self.enemies_template[enemy_type_1].clone());
        }
    }

    ///make sure count is always above 1
    pub fn update(&mut self, frequency: f32, count: i32) {
        //remove enemy if hp is 0 or below
        self.current_enemies.retain(|enemy| {
            if enemy.hp <= 0.0 {
                self.kill_count += 1;
                return false;
            } else {
                return true;
            }
        });

        //generate enemy every few x seconds
        self.counter += get_frame_time();

        if self.counter > frequency {
            self.generate(rand::thread_rng().gen_range(count - 1..count + 1) as i32);
            self.counter = 0.;
        }
    }

    pub fn clear(&mut self) {
        self.kill_count = 0;
        self.current_enemies.clear();
    }
}

// ///
// /// frequency   =   how often to generate the enemeis,
// /// count       =   how many enemies to generate at one specific time,
// /// total_count =   how many enemies in total to generate,
// pub fn generator(frequency: i32, count: i32, total_count: i32, startup_time_offset: i32) {

//     //logic 1
//     // frequency: every 5 seconds. count: 2 per
// }
