use crate::constants::{Sounds, ALIENT_SPAWN_Y, ALIEN_FRONT_SHOOT_SPACE, SCREEN_W};
use bevy::prelude::*;

use rand::{thread_rng, Rng};

use super::super::movement::Direction;
use super::shooter::{BulletShooter, Shooter, _can_shoot, _shoot};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AlienAnimation {
    Entering,
    MarchForward,
}

#[derive(Component, Debug, Copy, Clone)]
pub struct Alien {
    pub row: i16,
    pub dir: Direction,
    pub animation: Option<AlienAnimation>,

    // Shooter specific
    pub last_shot_ms: u128,
    pub shooting_rate: u128,
    pub shooting_speed: f32,
    pub movement_speed: f32,
    pub death_sound: &'static str,
}

impl Default for Alien {
    fn default() -> Self {
        let mut rng = thread_rng();

        Alien {
            dir: Direction::Left,
            row: 0,
            animation: Some(AlienAnimation::Entering),
            last_shot_ms: 0,
            shooting_rate: rng.gen_range(2000u128..=8000u128),
            shooting_speed: 40.0,
            movement_speed: 50.0,
            death_sound: Sounds::ALIEN_DEATH,
        }
    }
}

impl Alien {
    pub fn spawn_alien(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        x: f32,
        y: f32,
        init_last_shot_ms: u128,
    ) {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("alien_invader.png"),
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(0.6, 0.6, 1.0)),
                ..default()
            })
            .insert(Alien {
                last_shot_ms: init_last_shot_ms,
                // animation: Some(AlienAnimation::Entering),
                ..Default::default()
            });
    }

    pub fn spawn_fleet(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        c: i32,
        init_last_shot_ms: u128,
    ) {
        for n in 0..c {
            let x = SCREEN_W / -2.0 + (n as f32) * 50.0;
            Alien::spawn_alien(commands, asset_server, x, ALIENT_SPAWN_Y, init_last_shot_ms);
        }
    }
}

impl Shooter for Alien {
    fn can_shoot(&mut self, time_ms: u128) -> bool {
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
        let can_shoot = self.can_shoot(time.elapsed().as_millis());

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
