use crate::constants::PLAYER_FRONT_SHOOT_SPACE;
use bevy::prelude::*;

use super::shooter::{BulletShooter, Shooter, _can_shoot, _shoot};

#[derive(Debug, Component, Copy, Clone)]
pub struct Player {
    pub last_shot_ms: i128,
    // bigger = more shots/s
    pub shooting_rate: i128,
    pub shooting_speed: f32,
    pub movement_speed: f32,
}

impl Player {
    pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>, x: f32, y: f32) {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("space_ship.png"),
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            })
            .insert(Player {
                last_shot_ms: 0,
                shooting_rate: 500,
                shooting_speed: 100.0,
                movement_speed: 60.0,
            });
    }
}

impl Shooter for Player {
    fn can_shoot(&mut self, time_ms: i128) -> bool {
        _can_shoot(&mut self.last_shot_ms, self.shooting_rate, time_ms)
    }

    fn shoot(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        transform: &Transform,
        time: &Res<Time>,
    ) {
        let can_shoot = self.can_shoot(time.elapsed().as_millis() as i128);

        _shoot(
            commands,
            asset_server,
            transform,
            can_shoot,
            PLAYER_FRONT_SHOOT_SPACE,
            BulletShooter::Player(*self),
        )
    }
}
