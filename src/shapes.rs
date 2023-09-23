#[derive(Clone, Copy, Debug)]
struct Vec3f(f32, f32, f32);

impl Vec3f {
    fn subtract(&self, other: &Vec3f) -> Vec3f {
        Vec3f(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }

    fn dot(&self, other: &Vec3f) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    fn magnitude_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

struct Sphere {
    center: Vec3f,
    radius: f32,
}

impl Sphere {
    fn new(center: Vec3f, radius: f32) -> Sphere {
        Sphere { center, radius }
    }

    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
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
