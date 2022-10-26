use crate::{
    vec3::{scalar, Vec3},
    Color,
};

#[derive(Debug)]
pub struct Hit<'a> {
    pub material: &'a Material,
    pub dist: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

#[derive(Debug)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub specular_ratio: f64,
    pub diffuse_ratio: f64,
    pub shininess: f64,
    pub reflectiveness: f64,
    pub reflection_fuzziness: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub refraction_fuzziness: f64,
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
        let normal = (hit_point - self.center).normalize();

        Some(Hit {
            material: &self.material,
            dist: root,
            point: hit_point,
            normal,
        })
    }
}

#[derive(Debug)]
pub struct Checkerboard {
    pub y: f64,
    pub cell_size: f64,
    pub materials: (Material, Material),
}

impl Object for Checkerboard {
    fn intersect(&self, origin: &Vec3, dir: &Vec3) -> Option<Hit> {
        let dist = (self.y - origin.1) / dir.1;

        if dist < 0.001 {
            return None;
        }

        let hit_point = *origin + *dir * dist;
        let normal = Vec3(0.0, -dir.1, 0.0).normalize();

        let x_cell = f64::abs(f64::floor(hit_point.0 / self.cell_size) % 2.0) as u8;
        let z_cell = f64::abs(f64::floor(hit_point.2 / self.cell_size) % 2.0) as u8;

        Some(Hit {
            material: if (x_cell + z_cell) % 2 == 0 {
                &self.materials.0
            } else {
                &self.materials.1
            },
            dist,
            point: hit_point,
            normal,
        })
    }
}
