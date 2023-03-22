use bevy::prelude::*;

use crate::PrimaryCamera;

#[derive(Clone)]
pub struct ClearSceneEvent;

pub fn clear_scene_manager(
    mut commands: Commands,
    mut e: ResMut<Events<ClearSceneEvent>>,
    entities: Query<Entity, Without<PrimaryCamera>>,
) {
    if e.len() == 0 {
        return;
    }

    for _ in e.drain() {
        for entity in entities.iter() {
            commands.entity(entity).despawn()
        }
    }
}
