use crate::bvh::BvhNode;
use crate::shapes::{HitRecord, Shape};
use crate::vec3::Vec3f;
use rand::Rng;

pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
    pub radius: f32,
}

pub fn reflect(i: &Vec3f, n: &Vec3f) -> Vec3f {
    i.subtract(&n.multiply_scalar(2.0 * i.dot(n)))
}

pub fn refract(i: &Vec3f, n: &Vec3f, eta_t: f32, eta_i: f32) -> Vec3f {
    let cosi = -i.dot(n).clamp(-1.0, 1.0);
    if cosi < 0.0 {
        return refract(i, &n.negate(), eta_i, eta_t);
    }
    let eta = eta_i / eta_t;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        Vec3f(1.0, 0.0, 0.0)
    } else {
        i.multiply_scalar(eta)
            .add_ref(&n.multiply_scalar(eta * cosi - k.sqrt()))
    }
}

pub fn cast_ray(
    orig: &Vec3f,
    dir: &Vec3f,
    shapes: &[Box<dyn Shape>],
    bvh: &BvhNode,
    lights: &[Light],
    depth: i32,
    shadow_samples: u32,
) -> Vec3f {
    if depth > 4 {
        return Vec3f(0.2, 0.7, 0.8);
    }

    let Some(HitRecord {
        point,
        normal: n,
        material,
        ..
    }) = bvh.intersect(orig, dir, shapes)
    else {
        return Vec3f(0.2, 0.7, 0.8);
    };

    let reflect_dir = reflect(dir, &n).normalize();
    let refract_dir = refract(dir, &n, material.refractive_index, 1.0).normalize();

    let reflect_orig = if reflect_dir.dot(&n) < 0.0 {
        point.subtract(&n.multiply_scalar(1e-3))
    } else {
        point.add_ref(&n.multiply_scalar(1e-3))
    };
    let refract_orig = if refract_dir.dot(&n) < 0.0 {
        point.subtract(&n.multiply_scalar(1e-3))
    } else {
        point.add_ref(&n.multiply_scalar(1e-3))
    };

    let reflect_color = cast_ray(
        &reflect_orig,
        &reflect_dir,
        shapes,
        bvh,
        lights,
        depth + 1,
        shadow_samples,
    );
    let refract_color = cast_ray(
        &refract_orig,
        &refract_dir,
        shapes,
        bvh,
        lights,
        depth + 1,
        shadow_samples,
    );

    let mut diffuse_light_intensity = 0.0f32;
    let mut specular_light_intensity = 0.0f32;
    let mut rng = rand::rng();

    for light in lights {
        let samples = if light.radius > 0.0 {
            shadow_samples
        } else {
            1
        };
        let mut diff_accum = 0.0f32;
        let mut spec_accum = 0.0f32;

        for _ in 0..samples {
            let light_pos = if light.radius > 0.0 {
                let u: f32 = rng.random();
                let v: f32 = rng.random();
                let theta = 2.0 * std::f32::consts::PI * u;
                let phi = (2.0 * v - 1.0).acos();
                let offset = Vec3f(
                    light.radius * phi.sin() * theta.cos(),
                    light.radius * phi.sin() * theta.sin(),
                    light.radius * phi.cos(),
                );
                light.position.add_ref(&offset)
            } else {
                light.position
            };

            let light_dir = light_pos.subtract(&point).normalize();
            let light_distance = light_pos.subtract(&point).norm();

            let shadow_orig = if light_dir.dot(&n) < 0.0 {
                point.subtract(&n.multiply_scalar(1e-3))
            } else {
                point.add_ref(&n.multiply_scalar(1e-3))
            };

            let in_shadow = bvh
                .intersect(&shadow_orig, &light_dir, shapes)
                .is_some_and(|sh| sh.point.subtract(&shadow_orig).norm() < light_distance);

            if !in_shadow {
                diff_accum += light.intensity * f32::max(0.0, light_dir.dot(&n));
                spec_accum += light.intensity
                    * f32::powf(
                        f32::max(0.0, -reflect(&light_dir.negate(), &n).dot(dir)),
                        material.specular_exponent,
                    );
            }
        }

        diffuse_light_intensity += diff_accum / samples as f32;
        specular_light_intensity += spec_accum / samples as f32;
    }

    material
        .diffuse_color
        .multiply_scalar(diffuse_light_intensity * material.albedo[0])
        .add_ref(
            &Vec3f(1.0, 1.0, 1.0).multiply_scalar(specular_light_intensity * material.albedo[1]),
        )
        .add_ref(&reflect_color.multiply_scalar(material.albedo[2]))
        .add_ref(&refract_color.multiply_scalar(material.albedo[3]))
}
