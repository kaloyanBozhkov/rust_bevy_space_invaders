use bevy::{prelude::*, window::WindowResized};
pub mod constants;
mod entities;
mod events;
mod movement;
mod systems;
mod ui;

use constants::{SCREEN_H, SCREEN_W};
use systems::{
    init_setup::{init_setup, PrimaryCamera},
    systems_setup::systems_setup,
};

use events::events_setup::events_setup;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_system(adjust_screen_resize);

    events_setup(&mut app);
    systems_setup(&mut app);

    app.add_startup_system(init_setup).run();
}

fn adjust_screen_resize(
    mut resized: EventReader<WindowResized>,
    mut camera: Query<(&mut OrthographicProjection, With<PrimaryCamera>)>,
) {
    if let Some(resized) = resized.iter().last() {
        tracing::trace!("Resized! {:?}", resized);
        for (mut projection, _) in camera.iter_mut() {
            let width_scale = SCREEN_W / resized.width;
            let height_scale = SCREEN_H / resized.height;
            projection.scale = f32::max(width_scale, height_scale);
        }
    }
}
