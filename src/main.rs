use image;

use raytracer::objects;
use raytracer::vec3;

use raytracer::Scene;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const OUTPUT_PATH: &str = "output.png";

fn main() {
    let mut scene = Scene::new(1., image::Rgb([0, 128, 128]));

    let s1 = raytracer::objects::Sphere {
        center: vec3::Vec3::new(0.0, 0.0, -10.0),
        radius: 3.0,
    };

    scene.add_obj(Box::new(s1));
    let img = scene.render(WIDTH, HEIGHT);
    img.save(OUTPUT_PATH).unwrap();
}
