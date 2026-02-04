use crate::shapes::{HitRecord, Shape};
use crate::vec3::Vec3f;

#[derive(Clone, Debug)]
pub struct Aabb {
    pub min: Vec3f,
    pub max: Vec3f,
}

impl Aabb {
    pub fn new(min: Vec3f, max: Vec3f) -> Self {
        Aabb { min, max }
    }

    pub fn ray_intersect(&self, orig: &Vec3f, dir: &Vec3f) -> bool {
        let inv_d = Vec3f(1.0 / dir.0, 1.0 / dir.1, 1.0 / dir.2);

        let t0x = (self.min.0 - orig.0) * inv_d.0;
        let t1x = (self.max.0 - orig.0) * inv_d.0;
        let t0y = (self.min.1 - orig.1) * inv_d.1;
        let t1y = (self.max.1 - orig.1) * inv_d.1;
        let t0z = (self.min.2 - orig.2) * inv_d.2;
        let t1z = (self.max.2 - orig.2) * inv_d.2;

        let tmin = t0x.min(t1x).max(t0y.min(t1y)).max(t0z.min(t1z));
        let tmax = t0x.max(t1x).min(t0y.max(t1y)).min(t0z.max(t1z));

        tmax >= 0.0 && tmin <= tmax
    }

    pub fn surrounding(a: &Aabb, b: &Aabb) -> Aabb {
        Aabb {
            min: Vec3f(
                a.min.0.min(b.min.0),
                a.min.1.min(b.min.1),
                a.min.2.min(b.min.2),
            ),
            max: Vec3f(
                a.max.0.max(b.max.0),
                a.max.1.max(b.max.1),
                a.max.2.max(b.max.2),
            ),
        }
    }

    fn centroid(&self) -> Vec3f {
        Vec3f(
            (self.min.0 + self.max.0) * 0.5,
            (self.min.1 + self.max.1) * 0.5,
            (self.min.2 + self.max.2) * 0.5,
        )
    }

    fn longest_axis(&self) -> usize {
        let dx = self.max.0 - self.min.0;
        let dy = self.max.1 - self.min.1;
        let dz = self.max.2 - self.min.2;
        if dx > dy && dx > dz {
            0
        } else if dy > dz {
            1
        } else {
            2
        }
    }
}

pub enum BvhNode {
    Leaf {
        shape_idx: usize,
        aabb: Aabb,
    },
    Internal {
        left: Box<BvhNode>,
        right: Box<BvhNode>,
        aabb: Aabb,
    },
}

impl BvhNode {
    pub fn build(shapes: &[Box<dyn Shape>], indices: &mut [usize]) -> Self {
        assert!(!indices.is_empty());

        if indices.len() == 1 {
            let idx = indices[0];
            return BvhNode::Leaf {
                shape_idx: idx,
                aabb: shapes[idx].bounding_box(),
            };
        }

        let mut overall = shapes[indices[0]].bounding_box();
        for &idx in indices.iter().skip(1) {
            overall = Aabb::surrounding(&overall, &shapes[idx].bounding_box());
        }

        let axis = overall.longest_axis();
        indices.sort_by(|&a, &b| {
            let ca = shapes[a].bounding_box().centroid();
            let cb = shapes[b].bounding_box().centroid();
            let va = match axis {
                0 => ca.0,
                1 => ca.1,
                _ => ca.2,
            };
            let vb = match axis {
                0 => cb.0,
                1 => cb.1,
                _ => cb.2,
            };
            va.partial_cmp(&vb).unwrap()
        });

        let mid = indices.len() / 2;
        let (left_indices, right_indices) = indices.split_at_mut(mid);
        let left = BvhNode::build(shapes, left_indices);
        let right = BvhNode::build(shapes, right_indices);

        BvhNode::Internal {
            aabb: overall,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    pub fn intersect(
        &self,
        orig: &Vec3f,
        dir: &Vec3f,
        shapes: &[Box<dyn Shape>],
    ) -> Option<HitRecord> {
        match self {
            BvhNode::Leaf { shape_idx, aabb } => {
                if aabb.ray_intersect(orig, dir) {
                    shapes[*shape_idx].ray_intersect(orig, dir)
                } else {
                    None
                }
            }
            BvhNode::Internal { left, right, aabb } => {
                if !aabb.ray_intersect(orig, dir) {
                    return None;
                }
                let hit_left = left.intersect(orig, dir, shapes);
                let hit_right = right.intersect(orig, dir, shapes);
                match (hit_left, hit_right) {
                    (Some(l), Some(r)) => {
                        if l.t < r.t {
                            Some(l)
                        } else {
                            Some(r)
                        }
                    }
                    (Some(l), None) => Some(l),
                    (None, Some(r)) => Some(r),
                    (None, None) => None,
                }
            }
        }
    }
}
