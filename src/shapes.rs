mod vec3;
use vec3::Vec3f;

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

pub struct RecgtangularPrism {
    min: Vec3f,
    max: Vec3f,
}

impl RecgtangularPrism {
    pub fn new(min: Vec3f, max: Vec3f) -> RecgtangularPrism {
        RecgtangularPrism { min, max }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let t1 = (self.min.0 - orig.0) / dir.0;
        let t2 = (self.max.0 - orig.0) / dir.0;
        let t3 = (self.min.1 - orig.1) / dir.1;
        let t4 = (self.max.1 - orig.1) / dir.1;
        let t5 = (self.min.2 - orig.2) / dir.2;
        let t6 = (self.max.2 - orig.2) / dir.2;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return None;
        }

        let t = if tmin < 0.0 { tmax } else { tmin };
        Some(t)
    }
}
pub struct Cone {
    apex: Vec3f,
    height: f32,
    base_radius: f32,
}

impl Cone {
    pub fn new(apex: Vec3f, height: f32, base_radius: f32) -> Cone {
        Cone {
            apex,
            height,
            base_radius,
        }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let k = self.base_radius / self.height;

        let a = dir.0 * dir.0 + dir.2 * dir.2 - k * k * dir.1 * dir.1;
        let b = 2.0
            * (dir.0 * (orig.0 - self.apex.0) + dir.2 * (orig.2 - self.apex.2)
                - k * k * dir.1 * (orig.1 - self.apex.1));
        let c = (orig.0 - self.apex.0) * (orig.0 - self.apex.0)
            + (orig.2 - self.apex.2) * (orig.2 - self.apex.2)
            - k * k * (orig.1 - self.apex.1) * (orig.1 - self.apex.1);

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        // Check if the intersection points are within the cone's bounds (apex to base)
        let valid_t0 = (orig.1 + t0 * dir.1).between(self.apex.1, self.apex.1 + self.height);
        let valid_t1 = (orig.1 + t1 * dir.1).between(self.apex.1, self.apex.1 + self.height);

        if valid_t0 && valid_t1 {
            return Some(t0.min(t1));
        } else if valid_t0 {
            return Some(t0);
        } else if valid_t1 {
            return Some(t1);
        }

        None
    }
}

trait Between {
    fn between(self, min: f32, max: f32) -> bool;
}

impl Between for f32 {
    fn between(self, min: f32, max: f32) -> bool {
        self >= min && self <= max
    }
}
