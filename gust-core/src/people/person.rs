use crate::world::position::Position;

use super::skills::Skillset;

pub struct Person {
    name: String,
    position: Position,
    money: i64,
    health: f32,
    fatigue: f32,
    skillset: Skillset,
}
