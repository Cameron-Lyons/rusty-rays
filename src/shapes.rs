#![allow(dead_code)]

use crate::bvh::Aabb;
use crate::material::Material;
use crate::quartic::solve_quartic;
use crate::vec3::Vec3f;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3f,
    pub normal: Vec3f,
    pub material: Material,
}

pub trait Shape: Send + Sync {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord>;
    fn bounding_box(&self) -> Aabb;
}

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3f, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
        let l = self.center.subtract(orig);
        let tca = l.dot(dir);
        let d2 = l.magnitude_squared() - tca * tca;
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            return None;
        }
        let thc = (r2 - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return None;
        }
        let point = orig.add_ref(&dir.multiply_scalar(t0));
        let normal = point.subtract(&self.center).normalize();
        Some(HitRecord {
            t: t0,
            point,
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        let r = Vec3f(self.radius, self.radius, self.radius);
        Aabb::new(self.center.subtract(&r), self.center.add_ref(&r))
    }
}

pub struct RectangularPrism {
    min: Vec3f,
    max: Vec3f,
    material: Material,
}

impl RectangularPrism {
    pub fn new(min: Vec3f, max: Vec3f, material: Material) -> Self {
        RectangularPrism { min, max, material }
    }
}

impl Shape for RectangularPrism {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
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
        let point = orig.add_ref(&dir.multiply_scalar(t));
        let normal = aabb_normal(&point, &self.min, &self.max);
        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(self.min, self.max)
    }
}

pub struct Cube {
    center: Vec3f,
    side_length: f32,
    material: Material,
}

impl Cube {
    pub fn new(center: Vec3f, side_length: f32, material: Material) -> Self {
        Cube {
            center,
            side_length,
            material,
        }
    }

    fn half_side(&self) -> f32 {
        self.side_length / 2.0
    }

    fn min(&self) -> Vec3f {
        let h = self.half_side();
        Vec3f(self.center.0 - h, self.center.1 - h, self.center.2 - h)
    }

    fn max(&self) -> Vec3f {
        let h = self.half_side();
        Vec3f(self.center.0 + h, self.center.1 + h, self.center.2 + h)
    }
}

impl Shape for Cube {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
        let min = self.min();
        let max = self.max();

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
        let point = orig.add_ref(&dir.multiply_scalar(t));
        let normal = aabb_normal(&point, &min, &max);
        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(self.min(), self.max())
    }
}

pub struct Cone {
    apex: Vec3f,
    height: f32,
    base_radius: f32,
    material: Material,
}

impl Cone {
    pub fn new(apex: Vec3f, height: f32, base_radius: f32, material: Material) -> Self {
        Cone {
            apex,
            height,
            base_radius,
            material,
        }
    }
}

impl Shape for Cone {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
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

        let sqrt_disc = discriminant.sqrt();
        let t0 = (-b - sqrt_disc) / (2.0 * a);
        let t1 = (-b + sqrt_disc) / (2.0 * a);

        let y0 = orig.1 + t0 * dir.1;
        let y1 = orig.1 + t1 * dir.1;
        let valid_t0 = t0 > 0.0 && y0 >= self.apex.1 && y0 <= self.apex.1 + self.height;
        let valid_t1 = t1 > 0.0 && y1 >= self.apex.1 && y1 <= self.apex.1 + self.height;

        let t = if valid_t0 && valid_t1 {
            t0.min(t1)
        } else if valid_t0 {
            t0
        } else if valid_t1 {
            t1
        } else {
            return None;
        };

        let point = orig.add_ref(&dir.multiply_scalar(t));
        let dx = point.0 - self.apex.0;
        let dy = point.1 - self.apex.1;
        let dz = point.2 - self.apex.2;
        let normal = Vec3f(dx, -k * k * dy, dz).normalize();
        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(
            Vec3f(
                self.apex.0 - self.base_radius,
                self.apex.1,
                self.apex.2 - self.base_radius,
            ),
            Vec3f(
                self.apex.0 + self.base_radius,
                self.apex.1 + self.height,
                self.apex.2 + self.base_radius,
            ),
        )
    }
}

pub struct Cylinder {
    base_center: Vec3f,
    height: f32,
    radius: f32,
    material: Material,
}

impl Cylinder {
    pub fn new(base_center: Vec3f, height: f32, radius: f32, material: Material) -> Self {
        Cylinder {
            base_center,
            height,
            radius,
            material,
        }
    }
}

impl Shape for Cylinder {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
        let a = dir.0 * dir.0 + dir.2 * dir.2;
        let b =
            2.0 * (dir.0 * (orig.0 - self.base_center.0) + dir.2 * (orig.2 - self.base_center.2));
        let c = (orig.0 - self.base_center.0).powi(2) + (orig.2 - self.base_center.2).powi(2)
            - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();
        let t0 = (-b - sqrt_disc) / (2.0 * a);
        let t1 = (-b + sqrt_disc) / (2.0 * a);

        let y0 = orig.1 + t0 * dir.1;
        let y1 = orig.1 + t1 * dir.1;
        let valid_t0 =
            t0 > 0.0 && y0 >= self.base_center.1 && y0 <= self.base_center.1 + self.height;
        let valid_t1 =
            t1 > 0.0 && y1 >= self.base_center.1 && y1 <= self.base_center.1 + self.height;

        let t = if valid_t0 && valid_t1 {
            t0.min(t1)
        } else if valid_t0 {
            t0
        } else if valid_t1 {
            t1
        } else {
            return None;
        };

        let point = orig.add_ref(&dir.multiply_scalar(t));
        let normal = Vec3f(
            point.0 - self.base_center.0,
            0.0,
            point.2 - self.base_center.2,
        )
        .normalize();
        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(
            Vec3f(
                self.base_center.0 - self.radius,
                self.base_center.1,
                self.base_center.2 - self.radius,
            ),
            Vec3f(
                self.base_center.0 + self.radius,
                self.base_center.1 + self.height,
                self.base_center.2 + self.radius,
            ),
        )
    }
}

pub struct Pyramid {
    base_center: Vec3f,
    height: f32,
    half_base_length: f32,
    material: Material,
}

impl Pyramid {
    pub fn new(base_center: Vec3f, height: f32, half_base_length: f32, material: Material) -> Self {
        Pyramid {
            base_center,
            height,
            half_base_length,
            material,
        }
    }
}

impl Shape for Pyramid {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
        let epsilon = 1e-6;
        let mut best_t = f32::MAX;
        let mut best_normal = Vec3f(0.0, -1.0, 0.0);

        if dir.1.abs() > epsilon {
            let t_base = (self.base_center.1 - orig.1) / dir.1;
            if t_base > epsilon {
                let p = orig.add_ref(&dir.multiply_scalar(t_base));
                if p.0 >= self.base_center.0 - self.half_base_length
                    && p.0 <= self.base_center.0 + self.half_base_length
                    && p.2 >= self.base_center.2 - self.half_base_length
                    && p.2 <= self.base_center.2 + self.half_base_length
                {
                    best_t = t_base;
                    best_normal = Vec3f(0.0, -1.0, 0.0);
                }
            }
        }

        let apex = Vec3f(
            self.base_center.0,
            self.base_center.1 + self.height,
            self.base_center.2,
        );
        let h = self.half_base_length;
        let base_points = [
            Vec3f(
                self.base_center.0 - h,
                self.base_center.1,
                self.base_center.2 - h,
            ),
            Vec3f(
                self.base_center.0 + h,
                self.base_center.1,
                self.base_center.2 - h,
            ),
            Vec3f(
                self.base_center.0 + h,
                self.base_center.1,
                self.base_center.2 + h,
            ),
            Vec3f(
                self.base_center.0 - h,
                self.base_center.1,
                self.base_center.2 + h,
            ),
        ];

        for i in 0..4 {
            let v0 = apex;
            let v1 = base_points[i];
            let v2 = base_points[(i + 1) % 4];

            let edge1 = v1.subtract(&v0);
            let edge2 = v2.subtract(&v0);
            let hv = dir.cross(&edge2);
            let a = edge1.dot(&hv);

            if a > -epsilon && a < epsilon {
                continue;
            }

            let f = 1.0 / a;
            let s = orig.subtract(&v0);
            let u = f * s.dot(&hv);

            if !(0.0..=1.0).contains(&u) {
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
                best_normal = edge1.cross(&edge2).normalize();
            }
        }

        if best_t < f32::MAX {
            let point = orig.add_ref(&dir.multiply_scalar(best_t));
            Some(HitRecord {
                t: best_t,
                point,
                normal: best_normal,
                material: self.material,
            })
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        let h = self.half_base_length;
        Aabb::new(
            Vec3f(
                self.base_center.0 - h,
                self.base_center.1,
                self.base_center.2 - h,
            ),
            Vec3f(
                self.base_center.0 + h,
                self.base_center.1 + self.height,
                self.base_center.2 + h,
            ),
        )
    }
}

pub struct Ovoid {
    center: Vec3f,
    radii: Vec3f,
    material: Material,
}

impl Ovoid {
    pub fn new(center: Vec3f, radii: Vec3f, material: Material) -> Self {
        Ovoid {
            center,
            radii,
            material,
        }
    }
}

impl Shape for Ovoid {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
        let dir_n = Vec3f(
            dir.0 / self.radii.0,
            dir.1 / self.radii.1,
            dir.2 / self.radii.2,
        );
        let orig_s = Vec3f(
            (orig.0 - self.center.0) / self.radii.0,
            (orig.1 - self.center.1) / self.radii.1,
            (orig.2 - self.center.2) / self.radii.2,
        );

        let a = dir_n.dot(&dir_n);
        let b = 2.0 * dir_n.dot(&orig_s);
        let c = orig_s.dot(&orig_s) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();
        let t0 = (-b - sqrt_disc) / (2.0 * a);
        let t1 = (-b + sqrt_disc) / (2.0 * a);

        let t = if t0 > 0.0 {
            t0
        } else if t1 > 0.0 {
            t1
        } else {
            return None;
        };

        let point = orig.add_ref(&dir.multiply_scalar(t));
        let p = point.subtract(&self.center);
        let normal = Vec3f(
            p.0 / (self.radii.0 * self.radii.0),
            p.1 / (self.radii.1 * self.radii.1),
            p.2 / (self.radii.2 * self.radii.2),
        )
        .normalize();
        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(
            self.center.subtract(&self.radii),
            self.center.add_ref(&self.radii),
        )
    }
}

pub struct Torus {
    center: Vec3f,
    tube_radius: f32,
    torus_radius: f32,
    material: Material,
}

impl Torus {
    pub fn new(center: Vec3f, tube_radius: f32, torus_radius: f32, material: Material) -> Self {
        Torus {
            center,
            tube_radius,
            torus_radius,
            material,
        }
    }
}

impl Shape for Torus {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
        let p = orig.subtract(&self.center);
        let (x, y, z) = (p.0, p.1, p.2);
        let (xd, yd, zd) = (dir.0, dir.1, dir.2);
        let c2 = self.torus_radius;
        let a2 = self.tube_radius;

        let coeffs = [
            1.0,
            4.0 * (x * xd + y * yd),
            4.0 * (x * x + y * y) + 2.0 * (xd * xd + yd * yd) - a2 + c2 - 2.0 * c2 * zd * zd,
            4.0 * (x * x * xd + y * y * yd) - 4.0 * a2 * zd,
            x * x * x * x - 2.0 * a2 * (c2 - z * z)
                + (x * x + y * y + z * z + c2 - a2) * (x * x + y * y + z * z + c2 - a2),
        ];

        let roots = solve_quartic(&coeffs);
        let mut min_t: Option<f32> = None;
        for root in roots {
            if root > 1e-4 {
                min_t = Some(min_t.map_or(root, |cur| root.min(cur)));
            }
        }

        let t = min_t?;
        let point = orig.add_ref(&dir.multiply_scalar(t));
        let pp = point.subtract(&self.center);
        let dist_xy = (pp.0 * pp.0 + pp.1 * pp.1).sqrt();
        let normal = if dist_xy > 1e-6 {
            let r = self.torus_radius.abs().sqrt();
            let ring = Vec3f(pp.0 * r / dist_xy, pp.1 * r / dist_xy, 0.0);
            pp.subtract(&ring).normalize()
        } else {
            Vec3f(0.0, 0.0, 1.0)
        };
        Some(HitRecord {
            t,
            point,
            normal,
            material: self.material,
        })
    }

    fn bounding_box(&self) -> Aabb {
        let extent = self.torus_radius.abs() + self.tube_radius.abs() + 1.0;
        Aabb::new(
            Vec3f(
                self.center.0 - extent,
                self.center.1 - extent,
                self.center.2 - extent,
            ),
            Vec3f(
                self.center.0 + extent,
                self.center.1 + extent,
                self.center.2 + extent,
            ),
        )
    }
}

pub struct CheckerFloor {
    y: f32,
    x_range: (f32, f32),
    z_range: (f32, f32),
}

impl CheckerFloor {
    pub fn new(y: f32, x_range: (f32, f32), z_range: (f32, f32)) -> Self {
        CheckerFloor {
            y,
            x_range,
            z_range,
        }
    }
}

impl Shape for CheckerFloor {
    fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> Option<HitRecord> {
        if dir.1.abs() < 1e-3 {
            return None;
        }
        let t = (self.y - orig.1) / dir.1;
        if t < 1e-3 {
            return None;
        }
        let point = orig.add_ref(&dir.multiply_scalar(t));
        if point.0 < self.x_range.0
            || point.0 > self.x_range.1
            || point.2 < self.z_range.0
            || point.2 > self.z_range.1
        {
            return None;
        }
        let diffuse_color = if ((0.5 * point.0 + 1000.0) as i32 + (0.5 * point.2) as i32) & 1 == 0 {
            Vec3f(0.3, 0.3, 0.3)
        } else {
            Vec3f(0.3, 0.2, 0.1)
        };
        Some(HitRecord {
            t,
            point,
            normal: Vec3f(0.0, 1.0, 0.0),
            material: Material {
                refractive_index: 1.0,
                albedo: [1.0, 0.0, 0.0, 0.0],
                diffuse_color,
                specular_exponent: 0.0,
            },
        })
    }

    fn bounding_box(&self) -> Aabb {
        Aabb::new(
            Vec3f(self.x_range.0, self.y - 0.01, self.z_range.0),
            Vec3f(self.x_range.1, self.y + 0.01, self.z_range.1),
        )
    }
}

fn aabb_normal(point: &Vec3f, min: &Vec3f, max: &Vec3f) -> Vec3f {
    let eps = 1e-3;
    if (point.0 - min.0).abs() < eps {
        Vec3f(-1.0, 0.0, 0.0)
    } else if (point.0 - max.0).abs() < eps {
        Vec3f(1.0, 0.0, 0.0)
    } else if (point.1 - min.1).abs() < eps {
        Vec3f(0.0, -1.0, 0.0)
    } else if (point.1 - max.1).abs() < eps {
        Vec3f(0.0, 1.0, 0.0)
    } else if (point.2 - min.2).abs() < eps {
        Vec3f(0.0, 0.0, -1.0)
    } else {
        Vec3f(0.0, 0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::IVORY;

    #[test]
    fn test_sphere_hit() {
        let s = Sphere::new(Vec3f(0.0, 0.0, -5.0), 1.0, IVORY);
        let orig = Vec3f(0.0, 0.0, 0.0);
        let dir = Vec3f(0.0, 0.0, -1.0);
        let hit = s.ray_intersect(&orig, &dir);
        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!((hit.t - 4.0).abs() < 1e-3);
    }

    #[test]
    fn test_sphere_miss() {
        let s = Sphere::new(Vec3f(0.0, 0.0, -5.0), 1.0, IVORY);
        let orig = Vec3f(0.0, 0.0, 0.0);
        let dir = Vec3f(0.0, 1.0, 0.0);
        assert!(s.ray_intersect(&orig, &dir).is_none());
    }

    #[test]
    fn test_sphere_inside() {
        let s = Sphere::new(Vec3f(0.0, 0.0, 0.0), 5.0, IVORY);
        let orig = Vec3f(0.0, 0.0, 0.0);
        let dir = Vec3f(0.0, 0.0, -1.0);
        let hit = s.ray_intersect(&orig, &dir);
        assert!(hit.is_some());
        assert!(hit.unwrap().t > 0.0);
    }
}
