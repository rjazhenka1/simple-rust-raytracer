use raytracer::objects::{Checkerboard, Light, Sphere};
use raytracer::{Color, Scene, Vec3};

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const MAX_BOUNCES: u32 = 4;
const ANTIALIASING_SAMPLES: u32 = 128;
const OUTPUT_PATH: &str = "output.png";

mod materials {
    use raytracer::color::Color;
    use raytracer::objects::Material;

    pub const VERY_MATTE_RED: Material = Material {
        color: Color(0.8, 0.12, 0.12),
        specular_ratio: 0.0,
        diffuse_ratio: 0.3,
        shininess: 1.0,
        reflectiveness: 0.0,
        reflection_fuzziness: 1.0,
        refraction_fuzziness: 0.00,
        refractive_index: 1.0,
        transparency: 0.0,
    };

    pub const MATTE_GRAY: Material = Material {
        color: Color(0.5, 0.5, 0.5),
        specular_ratio: 0.05,
        diffuse_ratio: 1.0,
        shininess: 3.0,
        reflectiveness: 0.05,
        reflection_fuzziness: 0.5,
        refraction_fuzziness: 0.00,
        refractive_index: 1.0,
        transparency: 0.0,
    };

    pub const MATTE_BROWN: Material = Material {
        color: Color(0.5, 0.25, 0.2),
        specular_ratio: 0.05,
        diffuse_ratio: 1.0,
        shininess: 3.0,
        reflectiveness: 0.05,
        reflection_fuzziness: 0.5,
        refraction_fuzziness: 0.00,
        refractive_index: 1.0,
        transparency: 0.0,
    };

    pub const MATTE_BEIGE: Material = Material {
        color: Color(0.96, 0.96, 0.85),
        specular_ratio: 0.05,
        diffuse_ratio: 0.7,
        shininess: 3.0,
        reflectiveness: 0.05,
        reflection_fuzziness: 0.5,
        refraction_fuzziness: 0.00,
        refractive_index: 1.0,
        transparency: 0.0,
    };

    pub const GLOSSY_GREEN: Material = Material {
        color: Color(0.0, 0.4, 0.0),
        specular_ratio: 0.3,
        diffuse_ratio: 0.9,
        shininess: 30.0,
        reflectiveness: 0.2,
        reflection_fuzziness: 0.2,
        refraction_fuzziness: 0.00,
        refractive_index: 1.0,
        transparency: 0.0,
    };

    pub const BLURRY_MIRROR: Material = Material {
        color: Color(0.0, 0.0, 0.0),
        specular_ratio: 0.3,
        diffuse_ratio: 0.9,
        shininess: 30.0,
        reflectiveness: 0.95,
        reflection_fuzziness: 0.06,
        refractive_index: 1.0,
        refraction_fuzziness: 0.00,
        transparency: 0.0,
    };

    pub const GLASS: Material = Material {
        color: Color(0.0, 0.0, 0.0),
        specular_ratio: 0.3,
        diffuse_ratio: 0.9,
        shininess: 30.0,
        reflectiveness: 0.1,
        reflection_fuzziness: 0.08,
        refractive_index: 1.6,
        refraction_fuzziness: 0.0,
        transparency: 0.75,
    };
}

fn main() {
    let mut scene = Scene::new(0.7, Color(0.0, 0.5, 0.5));

    scene.add_light(Light {
        position: Vec3(0.0, -100.0, 0.0),
        intensity: 0.9,
    });

    scene.add_light(Light {
        position: Vec3(50.0, -50.0, 30.0),
        intensity: 0.8,
    });

    scene.add_obj(Box::new(Sphere {
        center: Vec3(-1.0, 0.0, -20.0),
        radius: 3.75,
        material: materials::MATTE_GRAY,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(-12.0, -5.0, -30.0),
        radius: 5.0,
        material: materials::VERY_MATTE_RED,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(4.0, 0.5, -15.0),
        radius: 2.5,
        material: materials::GLOSSY_GREEN,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(9.0, 0.5, -15.0),
        radius: 2.5,
        material: materials::GLASS,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(6.0, -5.0, -20.0),
        radius: 3.0,
        material: materials::BLURRY_MIRROR,
    }));

    scene.add_obj(Box::new(Sphere {
        center: Vec3(-2.0, 0.0, -12.0),
        radius: 1.0,
        material: materials::GLASS,
    }));

    scene.add_obj(Box::new(Checkerboard {
        y: 3.75,
        cell_size: 2.0,
        materials: (materials::MATTE_BEIGE, materials::MATTE_BROWN),
    }));

    let img = scene.render(WIDTH, HEIGHT, MAX_BOUNCES, ANTIALIASING_SAMPLES);
    img.save(OUTPUT_PATH).unwrap();
}
