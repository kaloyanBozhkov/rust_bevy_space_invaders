use bevy::prelude::Color;

pub const SCREEN_W: f32 = 400.0;
pub const SCREEN_H: f32 = 400.0;
// where bullet starts from
pub const PLAYER_FRONT_SHOOT_SPACE: f32 = 25.0;
pub const ALIEN_FRONT_SHOOT_SPACE: f32 = -55.0;
pub const ALIENT_SPAWN_Y: f32 = 215.;
pub const ALIENT_ROW_0_Y: f32 = 180.;

#[derive(Copy, Clone)]
pub struct Sounds;

impl Sounds {
    pub const PLAYER_DEATH: &'static str = "death.ogg";
    pub const ALIEN_DEATH: &'static str = "alien_death.ogg";
    pub const SHOOT_SOUND_1: &'static str = "shot.ogg";
}

pub struct Theme;

impl Theme {
    pub const PRIMARY_COLOR: Color = Color::rgb(0.886, 0.529, 0.262);
    pub const PRIMARY_COLOR_ACTIVE: Color = Color::rgb(0.925, 0.694, 0.521);
    pub const SECONDARY_COLOR: Color = Color::rgb(0.529, 0.262, 0.886);
}
