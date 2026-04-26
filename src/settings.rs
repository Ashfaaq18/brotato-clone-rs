use crate::input::AimMode;

pub struct Settings {
    pub aim_mode: AimMode,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            aim_mode: AimMode::Mouse,
        }
    }
}
