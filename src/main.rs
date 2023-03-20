use bevy::{prelude::*, window::WindowResized};
pub mod constants;
mod entities;
mod events;
mod movement;
mod systems;
mod ui;

use constants::{SCREEN_H, SCREEN_W};
use entities::{alien::Alien, player::Player};
use systems::{
    aliens::{aliens_move, aliens_shoot},
    bullet::move_bullets,
    player::{move_player, player_shoot},
};

use events::{
    score::{score_manager, ScoreEvent},
    sound::{sound_manager, SoundEvent},
};

use ui::score::UIText;

fn main() {
    App::new()
        .add_event::<ScoreEvent>()
        .add_system(score_manager)
        .add_event::<SoundEvent>()
        .add_system(sound_manager)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_player)
        .add_system(player_shoot)
        .add_system(aliens_shoot)
        .add_system(move_bullets)
        .add_system(aliens_move)
        .add_system(adjust_screen_resize)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PrimaryCamera);

    Player::spawn(&mut commands, &asset_server, 0.0, (SCREEN_H / -2.0) + 20.0);
    Alien::spawn_fleet(&mut commands, &asset_server, 10);
    UIText::score(&mut commands, &asset_server, 0);
}

#[derive(Debug, Component)]
struct PrimaryCamera;

fn adjust_screen_resize(
    mut resized: EventReader<WindowResized>,
    mut camera: Query<(&mut OrthographicProjection, With<PrimaryCamera>)>,
) {
    // let resize_list: Vec<_> = resized
    //     .iter()
    // // could have .cloned()
    //     .map(|resize| (resize.width, resize.height))
    //     .collect();

    // std::thread::spawn(move || {
    //     println!("{:?}", resize_list);
    // });

    // turbo fish
    // println!("{:?}", resized.iter().collect::<Vec<_>>());

    // @INVESTIGATE -> Dojo Lesson #2
    if let Some(resized) = resized.iter().last() {
        tracing::trace!("Resized! {:?}", resized);
        for (mut projection, _) in camera.iter_mut() {
            let width_scale = SCREEN_W / resized.width;
            let height_scale = SCREEN_H / resized.height;
            projection.scale = f32::max(width_scale, height_scale);
        }
    }
}
