use bevy::prelude::App;

use super::state::State;

pub fn resources_setup(app: &mut App) {
    app.insert_resource(State {
        ..Default::default()
    });
}
