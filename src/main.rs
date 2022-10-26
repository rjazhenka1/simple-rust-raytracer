use raytracer::objects::{Material, Sphere};
use raytracer::vec3::Vec3;

use raytracer::objects::Light;
use raytracer::Color;
use raytracer::Scene;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const MAX_BOUNCES: u32 = 5;
const ANTIALIASING_PASSES: u32 = 64;
const OUTPUT_PATH: &str = "output.png";

const VERY_MATTE_RED: Material = Material {
    color: Color(0.8, 0.12, 0.12),
    specular_ratio: 0.0,
    diffuse_ratio: 0.3,
    shininess: 1.0,
    reflectiveness: 0.0,
    reflection_fuzziness: 1.0,
    refractive_index: 1.0,
    transparency: 0.0,
};

const MATTE_GRAY: Material = Material {
    color: Color(0.5, 0.5, 0.5),
    specular_ratio: 0.2,
    diffuse_ratio: 1.0,
    shininess: 5.0,
    reflectiveness: 0.05,
    reflection_fuzziness: 0.5,
    refractive_index: 1.0,
    transparency: 0.0,
};

const GLOSSY_GREEN: Material = Material {
    color: Color(0.0, 0.4, 0.0),
    specular_ratio: 0.3,
    diffuse_ratio: 0.9,
    shininess: 30.0,
    reflectiveness: 0.1,
    reflection_fuzziness: 0.1,
    refractive_index: 1.0,
    transparency: 0.0,
};

const BLURRY_MIRROR: Material = Material {
    color: Color(0.0, 0.0, 0.0),
    specular_ratio: 0.3,
    diffuse_ratio: 0.9,
    shininess: 30.0,
    reflectiveness: 0.95,
    reflection_fuzziness: 0.18,
    refractive_index: 1.0,
    transparency: 0.0,
};

const GLASS: Material = Material {
    color: Color(0.0, 0.0, 0.0),
    specular_ratio: 0.3,
    diffuse_ratio: 0.9,
    shininess: 30.0,
    reflectiveness: 0.95,
    reflection_fuzziness: 0.0,
    refractive_index: 1.5,
    transparency: 0.9,
};

fn main() {
    let mut scene = Scene::new(0.7, Color(0.0, 0.5, 0.5));

    scene.add_light(Light {
        position: Vec3(3000.0, -100.0, 2000.0),
        intensity: 1.8,
    });

    scene.add_light(Light {
        position: Vec3(100.0, -100.0, 2000.0),
        intensity: 0.5,
    });

    scene.add_obj(Box::new(Sphere {
        center: Vec3(-1.0, 0.0, -20.0),
        radius: 3.75,
        material: MATTE_GRAY,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(-12.0, 0.0, -25.0),
        radius: 5.0,
        material: VERY_MATTE_RED,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(3.0, 2.5, -15.0),
        radius: 2.5,
        material: GLOSSY_GREEN,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(6.0, -5.0, -20.0),
        radius: 3.0,
        material: BLURRY_MIRROR,
    }));


    /* 

    scene.add_obj(Box::new(Sphere {
        center: Vec3(0.0, 10002.0, -15.0),
        radius: 10000.0,
        material: BLURRY_MIRROR,
    }));

    */

    let img = scene.render(WIDTH, HEIGHT, MAX_BOUNCES, ANTIALIASING_PASSES);
    img.save(OUTPUT_PATH).unwrap();
}
