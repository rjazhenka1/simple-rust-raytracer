pub mod objects;
pub mod vec3;
pub mod color;
pub use image::Rgb;
use objects::Light;
pub use vec3::{Vec3, scalar};
pub use color::Color; 


pub struct Scene {
    objs: Vec<Box<dyn objects::Object>>,
    lights: Vec<objects::Light>,
    fov: f64,
    bg: Color,
}

impl Scene {
    pub fn new(fov: f64, bg: Color) -> Self {
        Scene {
            objs: vec![],
            lights: vec![],
            fov,
            bg,
        }
    }
    pub fn add_obj(&mut self, obj: Box<dyn objects::Object>) {
        self.objs.push(obj);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    fn cast_single_ray(&self, origin: Vec3, dest: Vec3) -> Option<objects::Hit> {
        let mut closest: Option<objects::Hit> = None;

        for obj in &self.objs {
            if let Some(hit) = obj.intersect(&origin, &dest) {
                match closest {
                    Some(cl_hit) => {
                        if hit.dist < cl_hit.dist {
                            closest = Some(hit)
                        } else {
                            closest = Some(cl_hit)
                        }
                    }
                    None => closest = Some(hit),
                }
            }
        }

        closest
    }

    fn cast_ray(&self, origin: Vec3, dir: Vec3, limit: u32) -> Color {
        if limit == 0 {
            return self.bg;
        }

        if let Some(hit) = self.cast_single_ray(origin, dir) {
            let mut diffuse_light_intensity = 0.0;
            for light in &self.lights {
                let light_dir = (light.position - hit.normal_origin).norm();
                diffuse_light_intensity += f64::max(0.0, scalar(&hit.normal_dir, &light_dir)) * light.intensity;
                return hit.material.color * diffuse_light_intensity;
            }
            let diffuse_dir = hit.normal_origin + hit.normal_dir;
            return self.cast_ray(hit.normal_origin, diffuse_dir, limit - 1) / 2.0;
        }

        self.bg
    }

    pub fn render(&self, width: u32, height: u32) -> image::RgbImage {
        let mut img = image::RgbImage::new(width, height);

        for i in 0..width {
            for j in 0..height {
                let horizontal = f64::tan(self.fov);
                let vertical = f64::tan(self.fov) / width as f64 * height as f64;

                let x = horizontal * (-1.0 + ((2 * i) as f64 / (width - 1) as f64));
                let y = vertical * (-1.0 + ((2 * j) as f64 / (height - 1) as f64));
                let z = -1.0;

                let origin = Vec3(0.0, 0.0, 0.0);
                let dir = Vec3(x, y, z).norm();

                img.put_pixel(i, j, self.cast_ray(origin, dir, 50).into())
            }
        }

        img
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::Vec3;
    #[test]
    fn test_vec3_add() {
        let vec1 = Vec3(1.0, 5.0, 7.0);
        let vec2 = Vec3(-5.0, 10.0, 0.0);
        assert_eq!(Vec3(-4.0, 15.0, 7.0), vec1 + vec2);
    }

    #[test]
    fn test_vec3_sub() {
        let vec1 = Vec3(1.0, 5.0, 7.0);
        let vec2 = Vec3(1.0, 5.0, 0.0);
        assert_eq!(Vec3(0.0, 0.0, 7.0), vec1 - vec2);
    }

    #[test]
    fn test_vec3_neg() {
        let vec = Vec3(1.0, 2.0, -3.0);
        assert_eq!(Vec3(-1.0, -2.0, 3.0), -vec);
    }

    #[test]
    fn test_vec3_mul() {
        let vec = Vec3(1.0, 2.0, -3.0);
        assert_eq!(Vec3(0.0, 0.0, 0.0), vec * 0.0);
    }

    #[test]
    fn test_vec3_div() {
        let vec = Vec3(3.0, 3.0, -3.0);
        assert_eq!(Vec3(1.0, 1.0, -1.0), vec / 3.0);
    }

    #[test]
    fn test_vec3_norma() {
        let vec = Vec3(3.0, 0.0, 0.0);
        assert_eq!(Vec3(1.0, 0.0, 0.0), vec.norm());
    }

    #[test]
    fn test_vec3_eq() {
        let vec1 = Vec3(1.0, 1.0, 1.0);
        let vec2 = Vec3(1.0, 1.0, 1.0);
        assert!(vec1 == vec2);
    }
}
