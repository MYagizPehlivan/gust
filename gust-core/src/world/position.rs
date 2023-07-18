type Meters = f64;

/// Represents a geographical location with
/// a tuple containing latitude and longitude in radians.
pub struct Position(f64, f64);

impl Position {
    /// Constructs a new position with phi (latitude in radians)
    /// and lambda (longitude in radians).
    pub fn new(phi: f64, lambda: f64) -> Self {
        Self(phi, lambda)
    }

    pub fn r#move(&mut self, distance: Meters, destination: &Position) {
        todo!("Implement moving");
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}°, {:.3}°", self.0.to_degrees(), self.1.to_degrees())
    }
}
