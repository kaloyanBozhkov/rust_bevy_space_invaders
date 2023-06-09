use std::ops::Neg;

use crate::constants::SCREEN_H;

use super::constants::SCREEN_W;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Idle,
    Up,
    Down,
}

impl Direction {
    pub fn determine(left: bool, right: bool) -> Direction {
        match (left, right) {
            (true, false) => Direction::Left,
            (false, true) => Direction::Right,
            _ => Direction::Idle,
        }
    }
}
pub fn check_reached_end(transform: &mut Transform) -> bool {
    let mut reached_end = false;

    let edge = SCREEN_W - 80.;

    if transform.translation.x >= edge {
        transform.translation.x = edge;
        reached_end = true;
    } else if transform.translation.x <= edge.neg() {
        transform.translation.x = edge.neg();
        reached_end = true;
    }

    reached_end
}

pub fn x_move_subject(
    mut transform: &mut Transform,
    time: &Res<Time>,
    dir: Direction,
    speed: f32,
) -> bool {
    let amount = match dir {
        Direction::Left => -1.0,
        Direction::Right => 1.0,
        _ => 0.0,
    };

    transform.translation.x += speed * amount * time.delta_seconds();

    return check_reached_end(transform);
}

pub fn y_move_subject(
    transform: &mut Transform,
    time: &Res<Time>,
    dir: Direction,
    bullet_speed: f32,
) {
    let amount = match dir {
        Direction::Up => -1.0,
        Direction::Down => 1.0,
        _ => 0.0,
    };

    transform.translation.y -= bullet_speed * amount * time.delta_seconds();

    if transform.translation.y > SCREEN_H - 80.0 {
        transform.translation.y = SCREEN_H - 80.0;
    } else if transform.translation.y < -320.0 {
        transform.translation.y = -320.0;
    }
}

pub fn check_overlap(t1: &Transform, t2: &Transform, radius: f32) -> bool {
    f32::abs(t1.translation.x - t2.translation.x) <= radius
        && f32::abs(t1.translation.y - t2.translation.y) <= radius
}
