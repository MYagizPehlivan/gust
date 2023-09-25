use hexasphere::shapes::IcoSphere;

pub struct Cell {
    elevation: f32,
}

pub struct Globe {
    pub data: IcoSphere<Cell>,
}

impl Globe {
    pub fn new() -> Self {
        Self {
            data: IcoSphere::new(8, |_| Cell { elevation: 0.0 }),
        }
    }
}
