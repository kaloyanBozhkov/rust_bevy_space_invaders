pub const SCREEN_W: f32 = 400.0;
pub const SCREEN_H: f32 = 400.0;
// where bullet starts from
pub const PLAYER_FRONT_SHOOT_SPACE: f32 = 25.0;
pub const ALIEN_FRONT_SHOOT_SPACE: f32 = -55.0;

#[derive(Copy, Clone)]
pub struct Sounds;

impl Sounds {
    pub const PLAYER_DEATH: &'static str = "death.ogg";
    pub const ALIEN_DEATH: &'static str = "alien_death.ogg";
    pub const SHOOT_SOUND_1: &'static str = "shot.ogg";
}
