use crate::geometry::aabb::Aabb;
use crate::geometry::{Geometry, GeometryFile, Hittable};
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use glam::Vec3A;

use crate::error::{TracerError, TracerResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TranslateBuilder {
    pub object: Box<GeometryFile>,
    pub offset: Vec3A,
}

impl TryInto<Geometry> for TranslateBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Geometry> {
        Ok(Geometry::Translate(Translate::new(
            self.object.try_into()?,
            self.offset,
        )))
    }
}

pub struct Translate {
    pub object: Box<Geometry>,
    pub offset: Vec3A,
}

impl Translate {
    pub fn new(object: Box<Geometry>, offset: Vec3A) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin - self.offset, r.direction, r.time);
        if let Some(mut rec) = self.object.hit(&moved_r, t_min, t_max) {
            rec.position += self.offset;
            rec.set_face_normal(&moved_r, rec.normal);
            return Some(rec);
        }
        None
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        if let Some(output_box) = self.object.bounding_box(t0, t1) {
            Some(Aabb::new(
                output_box.min + self.offset,
                output_box.max + self.offset,
            ))
        } else {
            None
        }
    }
}
