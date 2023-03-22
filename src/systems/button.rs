use bevy::prelude::*;

use crate::{
    events::{clear_scene::ClearSceneEvent, start_game::StartGameEvent},
    ui::button::UIButton,
};

// use crate::constants::Theme;

pub fn button_system(
    mut interaction_query: Query<(&mut UIButton, &Interaction), With<UIButton>>,
    mut ev_start_game: EventWriter<StartGameEvent>,
    mut ev_clear_scene: EventWriter<ClearSceneEvent>,
) {
    for (mut btn, interaction) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if btn.clickable_once && btn.click_count > 0 {
                    return;
                }

                btn.click_count += 1;

                match btn.id.as_str() {
                    "start_game" => {
                        ev_clear_scene.send(ClearSceneEvent);
                        ev_start_game.send(StartGameEvent);
                    }
                    _ => unimplemented!(),
                };
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
