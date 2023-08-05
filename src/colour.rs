use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl Colour {
    pub const BLACK: Colour = Colour::new(0, 0, 0);
    pub const WHITE: Colour = Colour::new(255, 255, 255);
    pub const RED: Colour = Colour::new(255, 0, 0);
    pub const GREEN: Colour = Colour::new(0, 255, 0);
    pub const BLUE: Colour = Colour::new(0, 0, 255);
}

impl From<Colour> for u32 {
    fn from(value: Colour) -> Self {
        u32::from_be_bytes([0, value.r, value.g, value.b])
    }
}

impl Mul<Colour> for f32 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        Colour {
            r: (self * rhs.r as f32).round() as u8,
            g: (self * rhs.g as f32).round() as u8,
            b: (self * rhs.b as f32).round() as u8,
        }
    }
}
