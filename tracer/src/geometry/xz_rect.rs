use crate::geometry::aabb::Aabb;
use crate::geometry::Hittable;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;

use glam::Vec3A;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct XzRect {
    material_index: usize,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XzRect {
    pub fn new(x0: f32, x1: f32, z0: f32, z1: f32, k: f32, material_index: usize) -> Self {
        Self {
            material_index,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XzRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.y) / r.direction.y;
        if t < t_min as f32 || t > t_max as f32 {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let z = r.origin.z + t * r.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            r,
            Vec3A::new(0.0, 1.0, 0.0),
            self.material_index,
            (x - self.x0) / (self.x1 - self.x0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3A::new(self.x0, self.z0, self.k - 0.0001),
            Vec3A::new(self.x1, self.z1, self.k + 0.0001),
        ))
    }
}
