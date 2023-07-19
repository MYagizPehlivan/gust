use crate::world::position::Position;

pub enum Task {
    Idle,
    Traveling(Position),
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Task::Idle => write!(f, "Idle"),
            Task::Traveling(pos) => write!(f, "Traveling to {}", pos),
        }
    }
}
