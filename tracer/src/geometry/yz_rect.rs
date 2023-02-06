use crate::geometry::aabb::Aabb;
use crate::geometry::Hittable;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::Material;
use glam::Vec3A;
use std::rc::Rc;

pub struct YzRect {
    material_index: usize,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YzRect {
    pub fn new(y0: f32, y1: f32, z0: f32, z1: f32, k: f32, material_index: usize) -> Self {
        Self {
            material_index,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YzRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.x) / r.direction.x;
        if t < t_min as f32 || t > t_max as f32 {
            return None;
        }

        let y = r.origin.y + t * r.direction.y;
        let z = r.origin.z + t * r.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            r,
            Vec3A::new(1.0, 0.0, 0.0),
            self.material_index,
            (y - self.y0) / (self.y1 - self.y0),
            (z - self.z0) / (self.z1 - self.z0),
        ))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3A::new(self.y0, self.z0, self.k - 0.0001),
            Vec3A::new(self.y1, self.z1, self.k + 0.0001),
        ))
    }
}
