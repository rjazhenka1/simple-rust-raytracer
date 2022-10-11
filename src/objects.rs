use crate::vec3::{scalar, Vec3};

pub trait Object {
    fn intersects(&self, origin: &Vec3, dir: &Vec3) -> bool;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Object for Sphere {
    fn intersects(&self, origin: &Vec3, dir: &Vec3) -> bool {
        let ac = &(origin.clone() - self.center.clone());

        let a = scalar(&dir, &dir);
        let b = 2.0 * scalar(ac, &dir);
        let c = scalar(ac, ac) - self.radius * self.radius;

        b * b - 4.0 * a * c > 0.0
    }
}
