use crate::{
    constants::ALIENT_ROW_0_Y,
    entities::{
        alien::{Alien, AlienAnimation},
        shooter::Shooter,
    },
    movement::check_reached_end,
    resources::state::{GameStep, State},
};

use super::super::movement::{x_move_subject, Direction};
use bevy::prelude::*;

pub fn aliens_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut aliens: Query<(&mut Alien, &Transform), With<Alien>>,
    time: Res<Time>,
    state: Res<State>,
) {
    if state.step != GameStep::GameStarted {
        return;
    }

    for (mut alien, transform) in aliens.iter_mut() {
        alien.shoot(&mut commands, &asset_server, &audio, transform, &time)
    }
}

pub fn aliens_move(mut aliens: Query<(&mut Alien, &mut Transform), With<Alien>>, time: Res<Time>) {
    for (mut alien, mut transform) in aliens.iter_mut() {
        if alien.animation != None {
            continue;
        }

        let reached_end = check_reached_end(&mut transform);

        alien.dir = match (reached_end, alien.dir) {
            (true, Direction::Left) => Direction::Right,
            (true, Direction::Right) => Direction::Left,
            _ => alien.dir,
        };

        if reached_end {
            alien.row += 1;
            alien.animation = Some(AlienAnimation::MarchForward);
        }

        x_move_subject(&mut transform, &time, alien.dir, alien.movement_speed);
    }
}

pub fn fleet_incoming(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    aliens: Query<&Alien, With<Alien>>,
    time: Res<Time>,
    state: Res<State>,
) {
    if state.step != GameStep::GameStarted {
        return;
    }

    if aliens.iter().all(|alien| alien.row > 0) {
        Alien::spawn_fleet(&mut commands, &asset_server, 10, time.elapsed().as_millis());
    }
}

pub fn aliens_animate(
    mut aliens: Query<(&mut Alien, &mut Transform), With<Alien>>,
    time: Res<Time>,
) {
    for (mut alien, mut transform) in aliens.iter_mut() {
        if alien.animation == None {
            continue;
        }

        match alien.animation {
            Some(AlienAnimation::Entering) => {
                transform.translation.y -= 0.5;

                if transform.translation.y <= ALIENT_ROW_0_Y {
                    transform.translation.y = ALIENT_ROW_0_Y;
                    alien.animation = None;
                }
            }
            Some(AlienAnimation::MarchForward) => {
                let dest_y = ALIENT_ROW_0_Y - (alien.row as f32) * 30.;
                transform.translation.y -= alien.movement_speed * time.delta_seconds();

                if transform.translation.y <= dest_y {
                    transform.translation.y = dest_y;
                    alien.animation = None;
                }
            }
            _ => println!("weird"),
        }
    }
}
