use std::convert::From;
use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Color(pub f64, pub f64, pub f64);

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Color(x1, y1, z1) = self;
        let Color(x2, y2, z2) = rhs;
        Color(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let Color(x, y, z) = self;
        Color(x * rhs, y * rhs, z * rhs)
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let Color(x, y, z) = self;
        Color(x / rhs, y / rhs, z / rhs)
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(c: Color) -> Self {
        let Color(x, y, z) = c;
        image::Rgb([
            (f64::clamp(x, 0.0, 1.0) * 255.0) as u8,
            (f64::clamp(y, 0.0, 1.0) * 255.0) as u8,
            (f64::clamp(z, 0.0, 1.0) * 255.0) as u8,
        ])
    }
}
