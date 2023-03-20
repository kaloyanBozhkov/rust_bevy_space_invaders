use crate::{
    constants::SCREEN_H,
    entities::{alien::Alien, bullet::Bullet, player::Player, shooter::BulletShooter},
    events::{
        score::{ScoreEvent, ScoreOperation},
        sound::SoundEvent,
    },
    movement::{check_overlap, y_move_subject, Direction},
};

use bevy::prelude::*;

pub fn move_bullets(
    mut commands: Commands,
    mut bullets: Query<(&Bullet, Entity, &mut Transform), With<Bullet>>,
    mut targets: Query<
        (Entity, &mut Transform, (Option<&Player>, Option<&Alien>)),
        (Or<(With<Alien>, With<Player>)>, Without<Bullet>),
    >,
    mut ev_score: EventWriter<ScoreEvent>,
    mut ev_sound: EventWriter<SoundEvent>,
    time: Res<Time>,
) {
    let bullets_arr = bullets.iter_mut();

    if bullets_arr.len() == 0 {
        return;
    }

    for (b, b_entity, mut transform_bullet) in bullets_arr {
        let shooter = b.shooter;
        let (bullet_dir, bullet_speed) = match shooter {
            BulletShooter::Alien(shooter) => (Direction::Down, shooter.shooting_speed),
            BulletShooter::Player(shooter) => (Direction::Up, shooter.shooting_speed),
        };

        y_move_subject(&mut transform_bullet, &time, bullet_dir, bullet_speed);

        // if off screen remove bullet
        if transform_bullet.translation.y > (SCREEN_H / 2.0) {
            commands.entity(b_entity).despawn();
        }

        for (t_entity, transform_target, target) in targets.iter_mut() {
            // check intersection
            let overlap = check_overlap(&transform_bullet, &transform_target, 22.0);

            if overlap {
                commands.entity(t_entity).despawn();
                commands.entity(b_entity).despawn();

                let (t1, t2) = target;

                let (death_sound, op) = match (t1, t2) {
                    (Some(t1), _) => (t1.death_sound, ScoreOperation::RESET),
                    (_, Some(t2)) => (t2.death_sound, ScoreOperation::INC),

                    // below should never run, but keep in case more targets are added
                    _ => ("", ScoreOperation::RESET),
                };

                ev_sound.send(SoundEvent {
                    sound_file_name: death_sound.to_string(),
                    ..Default::default()
                });

                ev_score.send(ScoreEvent { op })
            }
        }

        // // check overlping bullets
        // for (_, b_2_entity, b_2_transform) in (bullets).iter() {
        //     // skip if position is exact => menas the bullet is checked against itself
        //     if b_2_transform.translation.x == transform_bullet.translation.x
        //         && b_2_transform.translation.y == transform_bullet.translation.y
        //     {
        //         return;
        //     }

        //     // check intersection
        //     let overlap = check_overlap(&transform_bullet, &b_2_transform, 22.0);

        //     if overlap {
        //         commands.entity(b_2_entity).despawn();
        //         commands.entity(b_entity).despawn();
        //     }
        // }
    }
}
