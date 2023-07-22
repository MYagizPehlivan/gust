use people::{person::Person, skills::Skillset, task::Task::Idle};
use world::{globe::Globe, position::Position};

pub struct Game {
    pub time_in_seconds: u64,
    pub player: Person,
    pub globe: Globe,
}

impl Game {
    pub fn new(time_in_seconds: u64) -> Self {
        Self {
            time_in_seconds,
            player: Person {
                name: "Alaric Gale".to_string(),
                position: Position::new(0.0, 0.0),
                money: 200,
                health: 100.0,
                fatigue: 0.0,
                skillset: Skillset::new(),
                task: Idle,
            },
            globe: Globe::new(),
        }
    }

    pub fn advance_state(&mut self, seconds: u64) {
        self.time_in_seconds += seconds;
    }
}

pub mod people;
pub mod util;
pub mod world;
