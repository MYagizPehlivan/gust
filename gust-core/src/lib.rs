pub struct Game {
    pub time_in_seconds: u64,
}

impl Game {
    pub fn advance_state(&mut self, seconds: u64) {
        self.time_in_seconds += seconds;
    }
}
