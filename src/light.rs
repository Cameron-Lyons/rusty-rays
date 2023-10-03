const NEAREST_DIST_THRESHOLD: f32 = 1e10;
const SMALL_NUMBER: f32 = 0.001;

pub struct Lights {
    pub sources: [Vec3f; 3],
}

impl Lights {
    pub fn reflect(&self, I: &Vec3f, N: &Vec3f) -> Vec3f {
        I.subtract(&N.multiply_scalar(2.0 * I.dot(N)))
    }

    pub fn refract(&self, I: &Vec3f, N: &Vec3f, eta_t: f32, eta_i: f32) -> Vec3f {
        let cosi = -f32::max(-1.0, f32::min(1.0, I.dot(N)));
        if cosi < 0.0 {
            return self.refract(I, &N.negate(), eta_i, eta_t);
        }
        let eta = eta_i / eta_t;
        let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
        if k < 0.0 {
            Vec3f(1.0, 0.0, 0.0)
        } else {
            I.multiply_scalar(eta)
                .add(&N.multiply_scalar(eta * cosi - k.sqrt()))
        }
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

pub fn scene_intersect(
    orig: &Vec3f,
    dir: &Vec3f,
    spheres: &[Sphere],
) -> (bool, Vec3f, Vec3f, Material) {
    let mut pt = Vec3f(0.0, 0.0, 0.0);
    let mut N = Vec3f(0.0, 0.0, 0.0);
    let mut material = Material {
        refractive_index: 1.0,
        albedo: [1.0; 4],
        diffuse_color: Vec3f(0.0, 0.0, 0.0),
        specular_exponent: 0.0,
    };

    let mut nearest_dist = 1e10;

    if dir.1.abs() > 0.001 {
        let d = -(orig.1 + 4.0) / dir.1;
        let p = orig.add(&dir.multiply_scalar(d));
        if d > 0.001 && d < nearest_dist && p.0.abs() < 10.0 && p.2 < -10.0 && p.2 > -30.0 {
            nearest_dist = d;
            pt = p;
            N = Vec3f(0.0, 1.0, 0.0);
            material.diffuse_color =
                if ((0.5 * pt.0 + 1000.0) as i32 + (0.5 * pt.2) as i32) & 1 == 0 {
                    Vec3f(0.3, 0.3, 0.3)
                } else {
                    Vec3f(0.3, 0.2, 0.1)
                };
        }
    }

    for s in spheres.iter() {
        let (intersection, d) = s.ray_intersect(orig, dir);
        if !intersection || d > nearest_dist {
            continue;
        }
        nearest_dist = d;
        pt = orig.add(&dir.multiply_scalar(nearest_dist));
        N = pt.subtract(&s.center);
        material = s.material;
    }

    (nearest_dist < 1000.0, pt, N, material)
}
