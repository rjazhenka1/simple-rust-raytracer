use raytracer::objects::{Material, Sphere};
use raytracer::vec3::Vec3;

use raytracer::Color;
use raytracer::objects::Light;
use raytracer::Scene;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const OUTPUT_PATH: &str = "output.png";

fn main() {
    let mut scene = Scene::new(0.7, Color(0.0, 0.5, 0.5));

    scene.add_light(Light {
        position: Vec3(0.0, 100.0, 30.0),
        intensity: 0.9
    });

    scene.add_obj(Box::new(Sphere {
        center: Vec3(0.0, 0.0, -20.0),
        radius: 3.0,
        material: Material {
            color: Color(0.5, 0.5, 0.5),
        },
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(5.0, 0.0, -25.0),
        radius: 5.0,
        material: Material {
            color: Color(0.8, 0.12, 0.12),
        },
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(3.0, 1.0, -15.0),
        radius: 2.5,
        material: Material {
            color: Color(0.0, 0.8, 0.0),
        },
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(-13.0, 5.0, -35.0),
        radius: 10.0,
        material: Material {
            color: Color(0.0, 0.7, 0.2),
        },
    }));

    let img = scene.render(WIDTH, HEIGHT);
    img.save(OUTPUT_PATH).unwrap();
}
