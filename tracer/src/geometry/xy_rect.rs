use crate::geometry::aabb::Aabb;
use crate::geometry::Hittable;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::Material;
use glam::Vec3A;
use std::rc::Rc;

pub struct XyRect {
    material_index: usize,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XyRect {
    pub fn new(x0: f32, x1: f32, y0: f32, y1: f32, k: f32, material_index: usize) -> Self {
        Self {
            material_index,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XyRect {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - r.origin.z) / r.direction.z;
        if t < t_min as f32 || t > t_max as f32 {
            return None;
        }

        let x = r.origin.x + t * r.direction.x;
        let y = r.origin.y + t * r.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        Some(HitRecord::new(
            t,
            r,
            Vec3A::new(0.0, 0.0, 1.0),
            self.material_index,
            (x - self.x0) / (self.x1 - self.x0),
            (y - self.y0) / (self.y1 - self.y0),
        ))
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3A::new(self.x0, self.y0, self.k - 0.0001),
            Vec3A::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}
