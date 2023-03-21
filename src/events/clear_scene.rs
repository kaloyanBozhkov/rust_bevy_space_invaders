use bevy::prelude::*;

use crate::PrimaryCamera;

#[derive(Clone)]
pub struct ClearSceneEvent;

pub fn clear_scene_manager(
    mut commands: Commands,
    mut ev_score: EventReader<ClearSceneEvent>,
    entities: Query<Entity, Without<PrimaryCamera>>,
) {
    for _ in ev_score.iter() {
        for entity in entities.iter() {
            commands.entity(entity).despawn()
        }
    }
}
