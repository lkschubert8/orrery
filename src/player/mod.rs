use crate::map;
use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
pub struct Player;
pub struct PlayerCollider;
#[derive(Debug)]
pub struct InputStack {
    pub directions: Vec<Direction>,
}
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct WalkTimer(pub Timer);

pub fn handle_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Translation)>,
) {
    for (_player, mut translation) in &mut query.iter() {
        let mut temp_translation = translation.0.clone();
        let velocity = 16.0 * 4.0;
        if keyboard_input.pressed(KeyCode::Up) {
            temp_translation += Vec3::new(0.0, 1.0, 0.0) * time.delta_seconds * velocity;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            temp_translation -= Vec3::new(0.0, 1.0, 0.0) * time.delta_seconds * velocity;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            temp_translation += Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds * velocity;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            temp_translation -= Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds * velocity;
        }
        translation.0 = temp_translation;
    }
}

pub fn handle_movement(
    time: Res<Time>,
    mut timer: ResMut<WalkTimer>,
    mut player_query: Query<(&Player, &mut Translation, &mut InputStack)>,
    mut terrain_query: Query<(&map::Collider, &Translation)>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_player, mut translation, mut command_stack) in &mut player_query.iter() {
            let player_size = 16.0 * 4.0;
            let mut initial_translation = (0, 0);

            for direction in &command_stack.directions {
                let movement = match direction {
                    Direction::Up => (0, 1),
                    Direction::Down => (0, -1),
                    Direction::Left => (-1, 0),
                    Direction::Right => (1, 0),
                };
                initial_translation.0 += movement.0;
                initial_translation.1 += movement.1;
            }
            let mut temp_translation = translation.0.clone();
            if i32::abs(initial_translation.0) > i32::abs(initial_translation.1) {
                if initial_translation.0 > 0 {
                    temp_translation += Vec3::new(1.0, 0.0, 0.0) * player_size;
                } else if initial_translation.0 < 0 {
                    temp_translation -= Vec3::new(1.0, 0.0, 0.0) * player_size;
                }
            } else {
                if initial_translation.1 > 0 {
                    temp_translation += Vec3::new(0.0, 1.0, 0.0) * player_size;
                } else if initial_translation.1 < 0 {
                    temp_translation -= Vec3::new(0.0, 1.0, 0.0) * player_size;
                }
            }
            for (collider, terrain_translation) in &mut terrain_query.iter() {
                if *collider == map::Collider::Blocked &&
                   temp_translation.x() == terrain_translation.0.x() &&
                   temp_translation.y() == terrain_translation.0.y(){
                        command_stack.directions = Vec::new();
                        return;
                   }
            }
            translation.0 = temp_translation;

            command_stack.directions = Vec::new();
        }
        timer.0.reset();
    }
}
