use bevy::prelude::*;

use crate::ui::button::UIButton;

#[derive(Clone)]
pub struct MainMenuEvent;

pub fn main_menu_manager(
    ev: EventReader<MainMenuEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if ev.len() == 0 {
        return;
    }

    ev.clear();

    UIButton::create_many(
        &mut commands,
        &asset_server,
        &[
            UIButton {
                id: "start_game".to_string(),
                text: "START GAME".to_string(),
                ..Default::default()
            },
            UIButton {
                id: "high_scores".to_string(),
                text: "HIGH SCORES".to_string(),
                ..Default::default()
            },
            UIButton {
                id: "quit_game".to_string(),
                text: "QUIT GAME".to_string(),
                ..Default::default()
            },
        ],
        Some(Size::new(Val::Px(250.), Val::Px(65.))),
    );
}
