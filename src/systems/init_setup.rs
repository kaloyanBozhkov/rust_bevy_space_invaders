use bevy::prelude::*;

use crate::events::main_menu::MainMenuEvent;

#[derive(Debug, Component)]
pub struct PrimaryCamera;

pub fn init_setup(mut commands: Commands, mut ev: EventWriter<MainMenuEvent>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PrimaryCamera);

    ev.send(MainMenuEvent {
        with_clear_screen: false,
    });
}
