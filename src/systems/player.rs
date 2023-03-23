use crate::{
    entities::{player::Player, shooter::Shooter},
    resources::state::{GameStep, State},
};

use super::super::movement::{x_move_subject, Direction};
use bevy::prelude::*;

pub fn move_player(
    key: Res<Input<KeyCode>>,
    mut players: Query<(&Player, &mut Transform), With<Player>>,
    time: Res<Time>,
    state: Res<State>,
) {
    if state.step != GameStep::GameStarted {
        return;
    }

    for (player, mut transform) in players.iter_mut() {
        let left = key.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = key.any_pressed([KeyCode::D, KeyCode::Right]);
        x_move_subject(
            &mut transform,
            &time,
            Direction::determine(left, right),
            player.movement_speed,
        );
    }
}

pub fn player_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    key: Res<Input<KeyCode>>,
    mut player: Query<(&mut Player, &Transform), With<Player>>,
    time: Res<Time>,
) {
    for (mut player, transform) in player.iter_mut() {
        let is_shooting = key.any_pressed([KeyCode::Space]);

        if !is_shooting {
            return;
        }

        player.shoot(&mut commands, &asset_server, &audio, transform, &time);
    }
}
