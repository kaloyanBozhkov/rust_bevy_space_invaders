use bevy::prelude::*;

use crate::{
    constants::SCREEN_H,
    entities::{alien::Alien, player::Player},
    resources::state::{GameStep, State},
    ui::ui_text::UIText,
};

#[derive(Clone)]
pub struct StartGameEvent;

pub fn start_game_manager(
    ev: EventReader<StartGameEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State>,
    time: Res<Time>,
) {
    if ev.len() == 0 {
        return;
    }

    // ensure no duplicate send
    ev.clear();

    state.step = GameStep::GameStarted;
    Player::spawn(&mut commands, &asset_server, 0.0, (SCREEN_H / -2.0) + 20.0);
    Alien::spawn_fleet(&mut commands, &asset_server, 10, time.elapsed().as_millis());
    UIText::score(&mut commands, &asset_server, 0);
}
