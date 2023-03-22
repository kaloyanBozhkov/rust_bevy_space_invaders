use crate::ui::ui_text::UIText;
use bevy::prelude::*;

use super::{clear_scene::ClearSceneEvent, main_menu::MainMenuEvent};

#[derive(PartialEq)]
pub enum ScoreOperation {
    INC,
    RESET,
}

pub struct ScoreEvent {
    pub op: ScoreOperation,
}

pub fn score_manager(
    mut ev_score: EventReader<ScoreEvent>,
    mut texts: Query<(&mut Text, &UIText), With<UIText>>,
    mut ev_main_menu: EventWriter<MainMenuEvent>,
    mut ev_clear_scene: EventWriter<ClearSceneEvent>,
) {
    for ev in ev_score.iter() {
        let (mut score, _) = texts
            .iter_mut()
            .find(|(_, text)| text.id == "score_count".to_string())
            .unwrap();
        let score_n = score.sections[0].value.parse::<i32>().unwrap();
        let val = match ev.op {
            ScoreOperation::INC => score_n + 1,
            ScoreOperation::RESET => 0,
        };

        score.sections[0].value = val.to_string();

        if ev.op == ScoreOperation::RESET {
            ev_clear_scene.send(ClearSceneEvent);
            ev_main_menu.send(MainMenuEvent);
        }
    }
}
