use bevy::prelude::Resource;

#[derive(PartialEq)]
pub enum GameStep {
    MainMenu,
    GameStarted,
}

#[derive(Resource)]
pub struct State {
    pub step: GameStep,
}

impl Default for State {
    fn default() -> Self {
        State {
            step: GameStep::MainMenu,
        }
    }
}
