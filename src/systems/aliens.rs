use crate::entities::{alien::Alien, shooter::Shooter};

use super::super::movement::{x_move_subject, Direction};
use bevy::prelude::*;

pub fn aliens_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut aliens: Query<(&mut Alien, &Transform), With<Alien>>,
    time: Res<Time>,
) {
    for (mut alien, transform) in aliens.iter_mut() {
        alien.shoot(&mut commands, &asset_server, transform, &time)
    }
}

pub fn aliens_move(mut aliens: Query<(&Alien, &mut Transform), With<Alien>>, time: Res<Time>) {
    for (alien, mut transform) in aliens.iter_mut() {
        x_move_subject(&mut transform, &time, Direction::Left, alien.movement_speed)
    }
}
