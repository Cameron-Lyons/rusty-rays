mod vec3;
use vec3::Vec3f;

#[derive(Clone, Copy, Debug)]
struct Material {
    refractive_index: f32,
    albedo: [f32; 4],
    diffuse_color: Vec3,
    specular_exponent: f32,
}

const IVORY: Material = Material {
    refractive_index: 1.0,
    albedo: [0.9, 0.5, 0.1, 0.0],
    diffuse_color: Vec3(0.4, 0.4, 0.3),
    specular_exponent: 50.0,
};

const GLASS: Material = Material {
    refractive_index: 1.5,
    albedo: [0.0, 0.9, 0.1, 0.8],
    diffuse_color: Vec3(0.6, 0.7, 0.8),
    specular_exponent: 125.0,
};

const RED_RUBBER: Material = Material {
    refractive_index: 1.0,
    albedo: [1.4, 0.3, 0.0, 0.0],
    diffuse_color: Vec3(0.3, 0.1, 0.1),
    specular_exponent: 10.0,
};

const MIRROR: Material = Material {
    refractive_index: 1.0,
    albedo: [0.0, 16.0, 0.8, 0.0],
    diffuse_color: Vec3(1.0, 1.0, 1.0),
    specular_exponent: 1425.0,
};

const METAL: Material = Material {
    refractive_index: 1.0,
    albedo: [0.7, 0.3, 0.1, 0.0],
    diffuse_color: Vec3(0.6, 0.6, 0.7),
    specular_exponent: 200.0,
};

const DARK_WOOD: Material = Material {
    refractive_index: 1.0,
    albedo: [0.8, 0.1, 0.05, 0.0],
    diffuse_color: Vec3(0.2, 0.1, 0.0),
    specular_exponent: 20.0,
};

const MARBLE: Material = Material {
    refractive_index: 1.5,
    albedo: [0.9, 0.2, 0.05, 0.0],
    diffuse_color: Vec3(0.7, 0.7, 0.9),
    specular_exponent: 100.0,
};

const GOLD: Material = Material {
    refractive_index: 0.47,
    albedo: [0.8, 1.0, 0.1, 0.0],
    diffuse_color: Vec3(1.0, 0.8, 0.0),
    specular_exponent: 300.0,
};

const VELVET: Material = Material {
    refractive_index: 1.0,
    albedo: [0.9, 0.1, 0.0, 0.0],
    diffuse_color: Vec3(0.5, 0.0, 0.5),
    specular_exponent: 5.0,
};

const CORTEN_STEEL: Material = Material {
    refractive_index: 2.5,
    albedo: [0.8, 0.3, 0.05, 0.0],
    diffuse_color: Vec3(0.7, 0.5, 0.4),
    specular_exponent: 20.0,
};
