use bevy::prelude::*;

pub struct WalkTimer(pub Timer);
#[derive(PartialEq, Debug)]
pub enum Collider {
    Level1,
    Level2,
    Blocked
}