use crate::{
    vec3::{scalar, Vec3},
    Color,
};

#[derive(Debug)]
pub struct Hit<'a> {
    pub material: &'a Material,
    pub dist: f64,
    pub normal_origin: Vec3,
    pub normal_dir: Vec3,
    pub front_face: bool,
}

#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

#[derive(Debug)]
pub struct Material {
    pub color: Color,
}

pub trait Object {
    fn intersect(&self, origin: &Vec3, dir: &Vec3) -> Option<Hit>;
}

#[derive(Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Object for Sphere {
    fn intersect(&self, origin: &Vec3, dir: &Vec3) -> Option<Hit> {
        let ac = &(*origin - self.center);

        let a = scalar(dir, dir);
        let b = scalar(ac, dir);
        let c = scalar(ac, ac) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-b - f64::sqrt(discriminant)) / a;
        if root < 0.001 {
            // too close due to rounding error
            root = (-b + f64::sqrt(discriminant)) / a;
        }

        if root < 0.001 {
            return None;
        }

        let hit_point = *origin + *dir * root;
        let normal = (hit_point - self.center / self.radius).norm();

        Some(Hit {
            material: &self.material,
            dist: root,
            normal_origin: hit_point,
            front_face: scalar(dir, &normal) < 0.0,
            normal_dir: if scalar(dir, &normal) < 0.0 {
                normal
            } else {
                -normal
            },
        })
    }
}
