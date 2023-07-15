use super::skills;

pub struct Player {
    money: i64,
    health: f32,
    fatigue: f32,
    skillset: skills::Skillset,
}
