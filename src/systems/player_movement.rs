use crate::entities;
use bevy::prelude::*;

pub fn handle_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&entities::Player, &mut Translation)>,
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