pub mod paths {
    pub const BACKGROUND_MAP: &str = "assets/background_map.png";
    pub const PLAYER_IDLE: &str = "assets\\topdown_shooter_assets\\sPlayerIdle_strip4.png";
    pub const PLAYER_RUN: &str = "assets\\topdown_shooter_assets\\sPlayerRun_strip7.png";
    pub const ENEMY_RUN: &str = "assets\\topdown_shooter_assets\\sEnemy_strip7.png";
    pub const GUN: &str = "assets\\topdown_shooter_assets\\sGun.png";
    pub const BULLET: &str = "assets\\topdown_shooter_assets\\sBullet.png";
    pub const UI_FONT: &str = "assets/ui_assets/The Bomb Sound.ttf";
}

#[derive(Clone, Copy)]
pub struct SpriteSheetSpec {
    pub path: &'static str,
    pub animation_name: &'static str,
    pub row: u32,
    pub frames: u32,
    pub fps: u32,
    pub tile_width: Option<u32>,
    pub tile_height: Option<u32>,
}

pub mod sprites {
    use super::{paths, SpriteSheetSpec};

    pub const PLAYER_IDLE: SpriteSheetSpec = SpriteSheetSpec {
        path: paths::PLAYER_IDLE,
        animation_name: paths::PLAYER_IDLE,
        row: 0,
        frames: 4,
        fps: 12,
        tile_width: Some(40),
        tile_height: Some(40),
    };

    pub const PLAYER_RUN: SpriteSheetSpec = SpriteSheetSpec {
        path: paths::PLAYER_RUN,
        animation_name: paths::PLAYER_RUN,
        row: 0,
        frames: 7,
        fps: 12,
        tile_width: Some(40),
        tile_height: Some(40),
    };

    pub const ENEMY_RUN: SpriteSheetSpec = SpriteSheetSpec {
        path: paths::ENEMY_RUN,
        animation_name: "run",
        row: 0,
        frames: 7,
        fps: 12,
        tile_width: None,
        tile_height: None,
    };
}
