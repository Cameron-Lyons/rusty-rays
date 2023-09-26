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

pub struct Box {
    min: Vec3f,
    max: Vec3f,
}

impl Box {
    pub fn new(min: Vec3f, max: Vec3f) -> Rectangle {
        Rectangle { min, max }
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
