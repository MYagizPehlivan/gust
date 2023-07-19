use crate::world::position::Position;

use super::{skills::Skillset, task::Task};

pub struct Person {
    pub name: String,
    pub position: Position,
    pub money: i64,
    pub health: f32,
    pub fatigue: f32,
    pub skillset: Skillset,
    pub task: Task,
}
