use bevy::prelude::*;

use super::{alien::Alien, bullet::Bullet, player::Player};

pub trait Shooter {
    fn can_shoot(&mut self, time_ms: i128) -> bool;
    fn shoot(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        audio: &Res<Audio>,
        transform: &Transform,
        time: &Res<Time>,
    );
}

pub fn _can_shoot(last_shot_ms: &mut i128, shooting_rate: i128, time_ms: i128) -> bool {
    let mut can_shoot = true;

    if ((*last_shot_ms) - time_ms).abs() < shooting_rate {
        can_shoot = false;
    }

    if can_shoot {
        (*last_shot_ms) = time_ms;
    }

    return can_shoot;
}

pub fn _shoot(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
    transform: &Transform,
    can_shoot: bool,
    front_shoot_space: f32,
    shooter: BulletShooter,
) {
    if !can_shoot {
        return;
    }

    let (x, y) = (
        transform.translation.x,
        transform.translation.y + front_shoot_space,
    );

    Bullet::spawn_bullet(commands, asset_server, audio, x, y, shooter);
}

#[derive(Debug, Copy, Clone)]
pub enum BulletShooter {
    Player(Player),
    Alien(Alien),
}
