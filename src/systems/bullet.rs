use crate::{
    constants::SCREEN_H,
    entities::{alien::Alien, bullet::Bullet, player::Player, shooter::BulletShooter},
    movement::{check_overlap, y_move_subject, Direction},
    ui::score::UIText,
};

use bevy::prelude::*;

pub fn move_bullets(
    mut commands: Commands,
    mut bullets: Query<(&Bullet, Entity, &mut Transform), With<Bullet>>,
    mut targets: Query<
        (Entity, &mut Transform),
        (Or<(With<Alien>, With<Player>)>, Without<Bullet>),
    >,
    mut texts: Query<(&mut Text, &UIText), With<UIText>>,
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

        for (a, transform_target) in targets.iter_mut() {
            // check intersection
            let overlap = check_overlap(&transform_bullet, &transform_target, 22.0);

            if overlap {
                commands.entity(a).despawn();
                commands.entity(b_entity).despawn();

                let (mut score, _) = texts
                    .iter_mut()
                    .find(|(_, text)| text.id == "score_count".to_string())
                    .unwrap();

                let score_n = score.sections[0].value.parse::<i32>().unwrap();
                score.sections[0].value = (score_n + 1).to_string();
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