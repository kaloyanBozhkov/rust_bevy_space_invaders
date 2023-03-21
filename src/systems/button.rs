use bevy::prelude::*;

use crate::{
    constants::SCREEN_H,
    entities::{alien::Alien, player::Player},
    events::clear_scene::ClearSceneEvent,
    ui::{button::UIButton, ui_text::UIText},
};

// use crate::constants::Theme;

pub fn button_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<(&mut UIButton, &Interaction), With<UIButton>>,
    mut ev_clear_scene: EventWriter<ClearSceneEvent>,
) {
    for (mut btn, interaction) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if btn.clickable_once && btn.click_count > 0 {
                    return;
                }

                btn.click_count += 1;

                UIEvents::call_method(
                    btn.id.as_str(),
                    &mut commands,
                    &asset_server,
                    &mut ev_clear_scene,
                );
            }
            Interaction::Hovered => {
                // btn.
                // *material = Handle::from_id(bevy::prelude::ColorMaterial::from(Color::GRAY).id());
            }
            Interaction::None => {
                // *material = Handle::from_id(bevy::prelude::ColorMaterial::from(Color::WHITE).id());
            }
        }
    }
}

pub struct UIEvents;

impl UIEvents {
    fn start_game(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        ev_clear_scene: &mut EventWriter<ClearSceneEvent>,
    ) {
        ev_clear_scene.send(ClearSceneEvent);
        Player::spawn(commands, &asset_server, 0.0, (SCREEN_H / -2.0) + 20.0);
        Alien::spawn_fleet(commands, &asset_server, 10);
        UIText::score(commands, &asset_server, 0);
    }

    pub fn call_method(
        method_name: &str,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        ev_clear_scene: &mut EventWriter<ClearSceneEvent>,
    ) {
        match method_name {
            "start_game" => UIEvents::start_game(commands, asset_server, ev_clear_scene),
            _ => unimplemented!(),
        }
    }
}
