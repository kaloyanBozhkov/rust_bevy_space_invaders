use bevy::prelude::App;

use super::{
    clear_scene::{clear_scene_manager, ClearSceneEvent},
    main_menu::{main_menu_manager, MainMenuEvent},
    score::{score_manager, ScoreEvent},
    sound::{sound_manager, SoundEvent},
    start_game::{start_game_manager, StartGameEvent},
};

pub fn events_setup(app: &mut App) {
    app.add_event::<ScoreEvent>()
        .add_system(score_manager)
        .add_event::<SoundEvent>()
        .add_system(clear_scene_manager)
        .add_event::<ClearSceneEvent>()
        .add_system(sound_manager)
        .add_event::<StartGameEvent>()
        .add_system(start_game_manager)
        .add_event::<MainMenuEvent>()
        .add_system(main_menu_manager);
}
