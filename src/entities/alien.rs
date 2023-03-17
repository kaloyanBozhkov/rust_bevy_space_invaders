use crate::constants::{Sounds, ALIEN_FRONT_SHOOT_SPACE, SCREEN_W};
use bevy::prelude::*;

use rand::{thread_rng, Rng};

use super::shooter::{BulletShooter, Shooter, _can_shoot, _shoot};

#[derive(Component, Debug, Copy, Clone)]
pub struct Alien {
    pub last_shot_ms: i128,
    pub shooting_rate: i128,
    pub shooting_speed: f32,
    pub movement_speed: f32,
    pub death_sound: &'static str,
}

impl Default for Alien {
    fn default() -> Self {
        let mut rng = thread_rng();

        Alien {
            last_shot_ms: 0,
            shooting_rate: rng.gen_range(2000.0..=8000.0) as i128,
            shooting_speed: 30.0,
            movement_speed: 50.0,
            death_sound: Sounds::ALIEN_DEATH,
        }
    }
}

impl Alien {
    pub fn spawn_alien(commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32) {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("alien_invader.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(Alien {
                ..Default::default()
            });
    }

    pub fn spawn_fleet(commands: &mut Commands, asset_server: &Res<AssetServer>, c: i32) {
        for n in 0..c {
            let x = SCREEN_W / -2.0 + (n as f32) * 50.0;
            Alien::spawn_alien(commands, asset_server, x, 180.0);
        }
    }
}

impl Shooter for Alien {
    fn can_shoot(&mut self, time_ms: i128) -> bool {
        _can_shoot(&mut self.last_shot_ms, self.shooting_rate, time_ms)
    }

    fn shoot(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        audio: &Res<Audio>,
        transform: &Transform,
        time: &Res<Time>,
    ) {
        let can_shoot = self.can_shoot(time.elapsed().as_millis() as i128);

        _shoot(
            commands,
            asset_server,
            audio,
            transform,
            can_shoot,
            ALIEN_FRONT_SHOOT_SPACE,
            BulletShooter::Alien(*self),
        )
    }
}
