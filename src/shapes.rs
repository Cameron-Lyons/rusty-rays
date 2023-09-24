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
