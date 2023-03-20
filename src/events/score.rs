use crate::ui::score::UIText;
use bevy::prelude::*;

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
    }
}
