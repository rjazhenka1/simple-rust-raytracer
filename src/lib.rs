pub mod color;
pub mod objects;
pub mod vec3;
pub use color::Color;
pub use image::Rgb;
use objects::Light;
pub use vec3::{reflect, scalar, Vec3, refract};

pub struct Scene {
    objs: Vec<Box<dyn objects::Object>>,
    lights: Vec<objects::Light>,
    fov: f64,
    bg: Color,
}

impl Scene {
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
            let mut specular_light_intensity = 0.0;

            let reflect_dir = (reflect(&dir, &hit.normal_dir) + Vec3::random_in_unit_sphere() * hit.material.reflection_fuzziness).normalize();
            let reflect_color = self.cast_ray(hit.point, reflect_dir, limit - 1);

            for light in &self.lights {
                let light_dir = (light.position - hit.point).normalize();
                let light_dist = light_dir.norm();

                if let Some(shadow_hit) = self.cast_single_ray(hit.point, light_dir) {
                    if (shadow_hit.point - hit.point).norm() < light_dist {
                        continue;
                    }
                }

                diffuse_light_intensity +=
                    f64::max(0.0, scalar(&hit.normal_dir, &light_dir)) * light.intensity;
                specular_light_intensity += f64::powf(
                    f64::max(
                        0.0,
                        scalar(&reflect(&light_dir, &hit.normal_dir).normalize(), &dir),
                    ),
                    hit.material.shininess * light.intensity,
                );
            }
            return hit.material.color * diffuse_light_intensity * hit.material.diffuse_ratio
                + Color(1.0, 1.0, 1.0) * specular_light_intensity * hit.material.specular_ratio
                + reflect_color * hit.material.reflectiveness;
        }

        self.bg
    }

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

    pub fn render(&self, width: u32, height: u32, max_it: u32, aa_passes: u32) -> image::RgbImage {
        let mut img = image::RgbImage::new(width, height);
        for i in 0..width {
            for j in 0..height {
                let mut color = Color(0.0, 0.0, 0.0);
                for _ in 0..aa_passes {
                    let horizontal = f64::tan(self.fov);
                    let vertical = f64::tan(self.fov) / width as f64 * height as f64;

                    let x = horizontal
                        * (-1.0 + (((2 * i) as f64 + fastrand::f64()) / (width - 1) as f64));
                    let y = vertical
                        * (-1.0 + (((2 * j) as f64 + fastrand::f64()) / (height - 1) as f64));
                    let z = -1.0;

                    let origin = Vec3(0.0, 0.0, 0.0);
                    let dir = Vec3(x, y, z).normalize();
                    color = color + self.cast_ray(origin, dir, max_it);
                }
                img.put_pixel(i, j, (color / aa_passes as f64).into())
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
        assert_eq!(Vec3(1.0, 0.0, 0.0), vec.normalize());
    }

    #[test]
    fn test_vec3_eq() {
        let vec1 = Vec3(1.0, 1.0, 1.0);
        let vec2 = Vec3(1.0, 1.0, 1.0);
        assert!(vec1 == vec2);
    }
}
