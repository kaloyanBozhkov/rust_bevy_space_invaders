use crate::entities::{alien::Alien, shooter::Shooter};

use super::super::movement::{x_move_subject, Direction};
use bevy::prelude::*;

pub fn aliens_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut aliens: Query<(&mut Alien, &Transform), With<Alien>>,
    time: Res<Time>,
) {
    for (mut alien, transform) in aliens.iter_mut() {
        alien.shoot(&mut commands, &asset_server, &audio, transform, &time)
    }
}

pub fn aliens_move(mut aliens: Query<(&Alien, &mut Transform), With<Alien>>, time: Res<Time>) {
    let mut dir = Direction::Left;

    for (alien, mut transform) in aliens.iter_mut() {
        let reached_end = x_move_subject(&mut transform, &time, dir, alien.movement_speed);

        dir = match (reached_end, dir) {
            (true, Direction::Left) => Direction::Right,
            (true, Direction::Right) => Direction::Left,
            _ => dir,
        }
    }
}
