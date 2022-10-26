use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn normalize(self) -> Vec3 {
        self / self.norm()
    }

    pub fn norm(&self) -> f64 {
        let Vec3(x, y, z) = self;
        f64::sqrt(x * x + y * y + z * z)
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        Vec3(
            -1.0 + 2.0 * fastrand::f64(),
            -1.0 + 2.0 * fastrand::f64(),
            -1.0 + 2.0 * fastrand::f64(),
        )
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let Vec3(x1, y1, z1) = self;
        let Vec3(x2, y2, z2) = rhs;
        Vec3(x1 + x2, y1 + y2, z1 + z2)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let Vec3(x, y, z) = self;
        Vec3(-x, -y, -z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let Vec3(x, y, z) = self;
        Vec3(x * rhs, y * rhs, z * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let Vec3(x, y, z) = self;
        Vec3(x / rhs, y / rhs, z / rhs)
    }
}

pub fn scalar(l: &Vec3, r: &Vec3) -> f64 {
    let Vec3(x1, y1, z1) = l;
    let Vec3(x2, y2, z2) = r;
    x1 * x2 + y1 * y2 + z1 * z2
}

pub fn reflect(r: &Vec3, normal: &Vec3) -> Vec3 {
    *r - *normal * scalar(r, normal) * 2.0
}

pub fn refract(i: &Vec3, normal: &Vec3, refractive_index: f64) -> Vec3 {
    let cos = f64::clamp(scalar(i, normal), -1.0, 1.0);
    let etai: f64;
    let etat: f64;
    let n: f64;
    todo!();
}
