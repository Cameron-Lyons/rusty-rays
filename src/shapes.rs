mod vec3;

struct Sphere {
    center: vec3::Vec3f,
    radius: f32,
}

impl Sphere {
    fn new(center: vec3::Vec3f, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    fn ray_intersect(&self, orig: &vec3::Vec3f, dir: &vec3::Vec3f) -> Option<f32> {
        let l = self.center.subtract(orig);
        let tca = l.dot(dir);
        let d2 = l.magnitude_squared() - tca * tca;
        if d2 > self.radius * self.radius {
            return None;
        }
        let thc = (self.radius * self.radius - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return None;
        }
        Some(t0)
    }
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, depth: i32) -> Vec3f {
    let (hit, point, n, material) = scene_intersect(orig, dir);
    if depth > 4 || !hit {
        return Vec3f(0.2, 0.7, 0.8); // background color
    }

    let reflect_dir = reflect(dir, &n).normalized();
    let refract_dir = refract(dir, &n, material.refractive_index).normalized();
    let reflect_color = cast_ray(&point, &reflect_dir, depth + 1);
    let refract_color = cast_ray(&point, &refract_dir, depth + 1);

    let mut diffuse_light_intensity = 0.0;
    let mut specular_light_intensity = 0.0;
    for light in &LIGHTS {
        let light_dir = light.subtract(&point).normalized();
        let (shadow_hit, shadow_pt, _, _) = scene_intersect(&point, &light_dir);
        if shadow_hit && (shadow_pt.subtract(&point).norm() < light.subtract(&point).norm()) {
            continue;
        }
        diffuse_light_intensity += f32::max(0.0, light_dir.dot(&n));
        specular_light_intensity += f32::powf(
            f32::max(0.0, -reflect(&light_dir.negate(), &n).dot(dir)),
            material.specular_exponent,
        );
    }
    material
        .diffuse_color
        .multiply_scalar(diffuse_light_intensity * material.albedo[0])
        .add(&Vec3f(1.0, 1.0, 1.0).multiply_scalar(specular_light_intensity * material.albedo[1]))
        .add(&reflect_color.multiply_scalar(material.albedo[2]))
        .add(&refract_color.multiply_scalar(material.albedo[3]))
}
