use crate::material::Material;
use crate::shapes::Sphere;
use crate::vec3::Vec3f;

pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
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
    spheres: &[Sphere],
    lights: &[Light],
    depth: i32,
) -> Vec3f {
    if depth > 4 {
        return Vec3f(0.2, 0.7, 0.8);
    }

    let (hit, point, n, material) = scene_intersect(orig, dir, spheres);
    if !hit {
        return Vec3f(0.2, 0.7, 0.8);
    }

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

    let reflect_color = cast_ray(&reflect_orig, &reflect_dir, spheres, lights, depth + 1);
    let refract_color = cast_ray(&refract_orig, &refract_dir, spheres, lights, depth + 1);

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;

    for light in lights {
        let light_dir = light.position.subtract(&point).normalize();
        let light_distance = light.position.subtract(&point).norm();

        let shadow_orig = if light_dir.dot(&n) < 0.0 {
            point.subtract(&n.multiply_scalar(1e-3))
        } else {
            point.add_ref(&n.multiply_scalar(1e-3))
        };

        let (shadow_hit, shadow_pt, _, _) = scene_intersect(&shadow_orig, &light_dir, spheres);
        if shadow_hit && shadow_pt.subtract(&shadow_orig).norm() < light_distance {
            continue;
        }

        diffuse_light_intensity += light.intensity * f32::max(0.0, light_dir.dot(&n));
        specular_light_intensity += light.intensity
            * f32::powf(
                f32::max(0.0, -reflect(&light_dir.negate(), &n).dot(dir)),
                material.specular_exponent,
            );
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

pub fn scene_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
) -> (bool, Vec3f, Vec3f, Material) {
    let mut pt = Vec3f(0.0, 0.0, 0.0);
    let mut n = Vec3f(0.0, 0.0, 0.0);
    let mut material = Material {
        refractive_index: 1.0,
        albedo: [1.0, 0.0, 0.0, 0.0],
        diffuse_color: Vec3f(0.0, 0.0, 0.0),
        specular_exponent: 0.0,
    };

    let mut nearest_dist: f32 = 1e10;

    if dir.1.abs() > 1e-3 {
        let d = -(orig.1 + 4.0) / dir.1;
        let p = orig.add_ref(&dir.multiply_scalar(d));
        if d > 1e-3 && d < nearest_dist && p.0.abs() < 10.0 && p.2 < -10.0 && p.2 > -30.0 {
            nearest_dist = d;
            pt = p;
            n = Vec3f(0.0, 1.0, 0.0);
            material.diffuse_color =
                if ((0.5 * pt.0 + 1000.0) as i32 + (0.5 * pt.2) as i32) & 1 == 0 {
                    Vec3f(0.3, 0.3, 0.3)
                } else {
                    Vec3f(0.3, 0.2, 0.1)
                };
        }
    }

    for s in spheres.iter() {
        if let Some(d) = s.ray_intersect(orig, dir)
            && d < nearest_dist
        {
            nearest_dist = d;
            pt = orig.add_ref(&dir.multiply_scalar(d));
            n = pt.subtract(&s.center).normalize();
            material = s.material;
        }
    }

    (nearest_dist < 1000.0, pt, n, material)
}
