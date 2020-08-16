use bevy::prelude::*;
pub struct Player;
#[derive(Debug)]
pub struct MovementCommandStack {
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

pub fn player_handle_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut MovementCommandStack)>,
) {
    for (_player, mut command_stack) in &mut query.iter() {
        let mut movement_stack_add: Vec<Direction> = Vec::new();
        if keyboard_input.pressed(KeyCode::Up) {
            movement_stack_add.push(Direction::Up);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            movement_stack_add.push(Direction::Down);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            movement_stack_add.push(Direction::Right);
        }
        if keyboard_input.pressed(KeyCode::Left) {
            movement_stack_add.push(Direction::Left);
        }
        command_stack.directions.append(&mut movement_stack_add);
    }
}

pub fn player_handle_movement(
    time: Res<Time>,
    mut timer: ResMut<WalkTimer>,
    mut player_query: Query<(&Player, &mut Translation, &mut MovementCommandStack)>,
) {
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        for (_player, mut translation, mut command_stack) in &mut player_query.iter() {
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
            if initial_translation.0 > 0 {
                translation.0 += Vec3::new(1.0, 0.0, 0.0) * 16.0 * 4.0;
            } else if initial_translation.0 < 0 {
                translation.0 -= Vec3::new(1.0, 0.0, 0.0) * 16.0 * 4.0;
            }

            if initial_translation.1 > 0 {
                translation.0 += Vec3::new(0.0, 1.0, 0.0) * 16.0 * 4.0;
            } else if initial_translation.1 < 0 {
                translation.0 -= Vec3::new(0.0, 1.0, 0.0) * 16.0 * 4.0;
            }
            command_stack.directions = Vec::new();
        }
        timer.0.reset();
    }
} 