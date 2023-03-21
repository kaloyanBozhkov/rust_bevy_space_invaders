use bevy::prelude::*;

use crate::{
    constants::SCREEN_H,
    entities::{alien::Alien, player::Player},
    ui::ui_text::UIText,
};

#[derive(Clone)]
pub struct StartGameEvent;

pub fn start_game_manager(
    ev: EventReader<StartGameEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if ev.len() == 0 {
        return;
    }

    Player::spawn(&mut commands, &asset_server, 0.0, (SCREEN_H / -2.0) + 20.0);
    Alien::spawn_fleet(&mut commands, &asset_server, 10);
    UIText::score(&mut commands, &asset_server, 0);
}
