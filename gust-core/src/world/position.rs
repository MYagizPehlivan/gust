type Meters = f64;

pub struct Position {
    /// Latitude in radians.
    phi: f64,

    /// Longitude in radians.
    lambda: f64,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}°, {:.3}°", self.phi.to_degrees(), self.lambda.to_degrees())
    }
}

impl Position {
    pub fn r#move(&mut self, distance: Meters, destination: &Position) {
        todo!("Implement moving");
    }
}
