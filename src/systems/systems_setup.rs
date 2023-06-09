use bevy::prelude::App;

use super::{
    aliens::{aliens_animate, aliens_move, aliens_shoot, fleet_incoming},
    bullet::move_bullets,
    button::button_system,
    player::{move_player, player_shoot},
};

pub fn systems_setup(app: &mut App) {
    app.add_system(move_player)
        .add_system(player_shoot)
        .add_system(aliens_shoot)
        .add_system(move_bullets)
        .add_system(aliens_move)
        .add_system(aliens_animate)
        .add_system(button_system)
        .add_system(fleet_incoming);
}
