pub trait Fractionable {
    fn fraction(self, f: f32) -> u16;
}

impl Fractionable for u16 {
    fn fraction(self, f: f32) -> u16 {
        (self as f32 * f) as u16
    }
}

pub mod stack;
