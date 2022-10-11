pub mod objects;
pub mod vec3;
use image;
use vec3::Vec3;

pub type Color = image::Rgb<u8>;

pub struct Scene {
    objs: Vec<Box<dyn objects::Object>>,
    fov: f64,
    bg: Color,
}

impl Scene {
    pub fn new(fov: f64, bg: Color) -> Self {
        Scene {
            objs: vec![],
            fov: fov,
            bg: bg,
        }
    }

    pub fn add_obj(&mut self, obj: Box<dyn objects::Object>) {
        self.objs.push(obj);
    }

    fn cast_ray(&self, origin: Vec3, dest: Vec3) -> Color {

        for obj in &self.objs {
            if obj.intersects(&origin, &dest) {
                return image::Rgb([255, 0, 0]);
            }
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
                
                let origin = Vec3::new(0.0, 0.0, 0.0);
                let dest = Vec3::new(x, y, z);

                img.put_pixel(i, j, self.cast_ray(origin, dest))
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
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(-5.0, 10.0, 0.0);
        assert_eq!(Vec3::new(-4.0, 15.0, 7.0), vec1 + vec2);
    }

    #[test]
    fn test_vec3_sub() {
        let vec1 = Vec3::new(1.0, 5.0, 7.0);
        let vec2 = Vec3::new(1.0, 5.0, 0.0);
        assert_eq!(Vec3::new(0.0, 0.0, 7.0), vec1 - vec2);
    }

    #[test]
    fn test_vec3_neg() {
        let vec = Vec3::new(1.0, 2.0, -3.0);
        assert_eq!(Vec3::new(-1.0, -2.0, 3.0), -vec);
    }

    #[test]
    fn test_vec3_mul() {
        let vec = Vec3::new(1.0, 2.0, -3.0);
        assert_eq!(Vec3::new(0.0, 0.0, 0.0), vec * 0.0);
    }

    #[test]
    fn test_vec3_div() {
        let vec = Vec3::new(3.0, 3.0, -3.0);
        assert_eq!(Vec3::new(1.0, 1.0, -1.0), vec / 3.0);
    }

    #[test]
    fn test_vec3_norma() {
        let vec = Vec3::new(3.0, 0.0, 0.0);
        assert_eq!(Vec3::new(1.0, 0.0, 0.0), vec.norm());
    }

    #[test]
    fn test_vec3_eq() {
        let vec1 = Vec3::new(1.0, 1.0, 1.0);
        let vec2 = Vec3::new(1.0, 1.0, 1.0);
        assert!(vec1 == vec2);
    }
}
