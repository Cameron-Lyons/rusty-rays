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

pub struct Cylinder {
    base_center: Vec3f,
    height: f32,
    radius: f32,
}

impl Cylinder {
    pub fn new(base_center: Vec3f, height: f32, radius: f32) -> Cylinder {
        Cylinder {
            base_center,
            height,
            radius,
        }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let a = dir.0 * dir.0 + dir.2 * dir.2;
        let b =
            2.0 * (dir.0 * (orig.0 - self.base_center.0) + dir.2 * (orig.2 - self.base_center.2));
        let c = (orig.0 - self.base_center.0) * (orig.0 - self.base_center.0)
            + (orig.2 - self.base_center.2) * (orig.2 - self.base_center.2)
            - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        let valid_t0 =
            (orig.1 + t0 * dir.1).between(self.base_center.1, self.base_center.1 + self.height);
        let valid_t1 =
            (orig.1 + t1 * dir.1).between(self.base_center.1, self.base_center.1 + self.height);

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

pub struct Pyramid {
    base_center: Vec3f,
    height: f32,
    half_base_length: f32,
}

impl Pyramid {
    pub fn new(base_center: Vec3f, height: f32, half_base_length: f32) -> Pyramid {
        Pyramid {
            base_center,
            height,
            half_base_length,
        }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let epsilon = 1e-6;

        // Intersection with base square
        let t_base = (self.base_center.1 - orig.1) / dir.1;
        if t_base >= 0.0 {
            let x = orig.0 + t_base * dir.0;
            let z = orig.2 + t_base * dir.2;
            if x.between(
                self.base_center.0 - self.half_base_length,
                self.base_center.0 + self.half_base_length,
            ) && z.between(
                self.base_center.2 - self.half_base_length,
                self.base_center.2 + self.half_base_length,
            ) {
                return Some(t_base);
            }
        }

        let apex = Vec3f(
            self.base_center.0,
            self.base_center.1 + self.height,
            self.base_center.2,
        );

        // Möller–Trumbore intersection algorithm for triangles
        let mut best_t = std::f32::MAX;
        let base_points = [
            Vec3f(
                self.base_center.0 - self.half_base_length,
                self.base_center.1,
                self.base_center.2 - self.half_base_length,
            ),
            Vec3f(
                self.base_center.0 + self.half_base_length,
                self.base_center.1,
                self.base_center.2 - self.half_base_length,
            ),
            Vec3f(
                self.base_center.0 + self.half_base_length,
                self.base_center.1,
                self.base_center.2 + self.half_base_length,
            ),
            Vec3f(
                self.base_center.0 - self.half_base_length,
                self.base_center.1,
                self.base_center.2 + self.half_base_length,
            ),
        ];

        for i in 0..4 {
            let v0 = apex;
            let v1 = base_points[i];
            let v2 = base_points[(i + 1) % 4];

            let edge1 = v1.subtract(&v0);
            let edge2 = v2.subtract(&v0);
            let h = dir.cross(&edge2);
            let a = edge1.dot(&h);

            if a > -epsilon && a < epsilon {
                continue; // Ray is parallel to triangle
            }

            let f = 1.0 / a;
            let s = orig.subtract(&v0);
            let u = f * s.dot(&h);

            if u < 0.0 || u > 1.0 {
                continue;
            }

            let q = s.cross(&edge1);
            let v = f * dir.dot(&q);

            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let t = f * edge2.dot(&q);
            if t > epsilon && t < best_t {
                best_t = t;
            }
        }

        if best_t < std::f32::MAX {
            return Some(best_t);
        }

        None
    }
}

pub struct Cube {
    center: Vec3f,
    side_length: f32,
}

impl Cube {
    pub fn new(center: Vec3f, side_length: f32) -> Cube {
        Cube {
            center,
            side_length,
        }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let half_side = self.side_length / 2.0;
        let min = Vec3f(
            self.center.0 - half_side,
            self.center.1 - half_side,
            self.center.2 - half_side,
        );
        let max = Vec3f(
            self.center.0 + half_side,
            self.center.1 + half_side,
            self.center.2 + half_side,
        );

        let t1 = (min.0 - orig.0) / dir.0;
        let t2 = (max.0 - orig.0) / dir.0;
        let t3 = (min.1 - orig.1) / dir.1;
        let t4 = (max.1 - orig.1) / dir.1;
        let t5 = (min.2 - orig.2) / dir.2;
        let t6 = (max.2 - orig.2) / dir.2;

        let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
        let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

        if tmax < 0.0 || tmin > tmax {
            return None;
        }

        let t = if tmin < 0.0 { tmax } else { tmin };
        Some(t)
    }
}

pub struct Ovoid {
    center: Vec3f,
    radii: Vec3f,
}

impl Ovoid {
    pub fn new(center: Vec3f, radii: Vec3f) -> Ovoid {
        Ovoid { center, radii }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<f32> {
        let dir_normalized = Vec3f(
            dir.0 / self.radii.0,
            dir.1 / self.radii.1,
            dir.2 / self.radii.2,
        );

        let orig_shifted = Vec3f(
            orig.0 - self.center.0,
            orig.1 - self.center.1,
            orig.2 - self.center.2,
        );

        let orig_normalized = Vec3f(
            orig_shifted.0 / self.radii.0,
            orig_shifted.1 / self.radii.1,
            orig_shifted.2 / self.radii.2,
        );

        let a = dir_normalized.dot(&dir_normalized);
        let b = 2.0 * dir_normalized.dot(&orig_normalized);
        let c = orig_normalized.dot(&orig_normalized) - 1.0;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t0 > t1 {
            return Some(t1);
        }

        Some(t0)
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
