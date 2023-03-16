use bevy::prelude::*;

use super::shooter::BulletShooter;

#[derive(Component, Debug)]
pub struct Bullet {
    pub shooter: BulletShooter,
    pub speed: f32,
}

impl Bullet {
    pub fn spawn_bullet(
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        x: f32,
        y: f32,
        shooter: BulletShooter,
    ) {
        let (bullet_rotation, bullet_speed) = match shooter {
            BulletShooter::Player(shooter) => (0.0_f32, shooter.shooting_speed),
            BulletShooter::Alien(shooter) => (180.0_f32, shooter.shooting_speed),
        };

        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("shot.png"),
                transform: Transform::from_xyz(x, y, 0.0)
                    .with_scale(Vec3::new(0.8, 0.8, 1.0))
                    .with_rotation(Quat::from_rotation_x(bullet_rotation.to_radians())),
                ..default()
            })
            .insert(Bullet {
                shooter,
                speed: bullet_speed,
            });
    }
}
