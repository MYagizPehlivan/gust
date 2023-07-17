use super::skills;

pub struct Person {
    name: String,
    money: i64,
    health: f32,
    fatigue: f32,
    skillset: skills::Skillset,
}
