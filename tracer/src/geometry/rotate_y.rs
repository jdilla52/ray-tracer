use crate::geometry::aabb::Aabb;
use crate::geometry::{Geometry, GeometryFile, Hittable};
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use glam::Vec3A;

use crate::error::{TracerError, TracerResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RotateYBuilder {
    pub object: Box<GeometryFile>,
    pub angle: f32,
}

impl TryInto<Geometry> for RotateYBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Geometry> {
        Ok(Geometry::RotateY(RotateY::new(
            self.object.try_into()?,
            self.angle,
        )))
    }
}

pub struct RotateY {
    pub object: Box<Geometry>,
    pub sin_theta: f32,
    pub cos_theta: f32,
    pub has_box: bool,
    pub bbox: Aabb,
}

impl RotateY {
    pub(crate) fn new(object: Box<Geometry>, angle: f32) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        if let Some(bbox) = object.bounding_box(0.0, 1.0) {
            let mut min = Vec3A::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
            let mut max = Vec3A::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max.x + (1 - i) as f32 * bbox.min.x;
                        let y = j as f32 * bbox.max.y + (1 - j) as f32 * bbox.min.y;
                        let z = k as f32 * bbox.max.z + (1 - k) as f32 * bbox.min.z;
                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;
                        let tester = Vec3A::new(newx, y, newz);
                        for c in 0..3 {
                            min[c] = min[c].min(tester[c]);
                            max[c] = max[c].max(tester[c]);
                        }
                    }
                }
            }
            Self {
                object,
                sin_theta,
                cos_theta,
                has_box: false,
                bbox: Aabb::new(min, max),
            }
        } else {
            log::info!("Null bounding box in RotateY constructor.");
            Self {
                object,
                sin_theta,
                cos_theta,
                has_box: false,
                bbox: Aabb::new(Vec3A::ZERO, Vec3A::ZERO),
            }
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let origin = Vec3A::new(
            self.cos_theta * r.origin.x - self.sin_theta * r.origin.z,
            r.origin.y,
            self.sin_theta * r.origin.x + self.cos_theta * r.origin.z,
        );
        let direction = Vec3A::new(
            self.cos_theta * r.direction.x - self.sin_theta * r.direction.z,
            r.direction.y,
            self.sin_theta * r.direction.x + self.cos_theta * r.direction.z,
        );

        let rotated_r = Ray::new(origin, direction, r.time);

        if let Some(mut rec) = self.object.hit(&rotated_r, t_min, t_max) {
            let p = Vec3A::new(
                self.cos_theta * rec.position.x + self.sin_theta * rec.position.z,
                rec.position.y,
                -self.sin_theta * rec.position.x + self.cos_theta * rec.position.z,
            );
            let normal = Vec3A::new(
                self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z,
                rec.normal.y,
                -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z,
            );
            rec.position = p;
            rec.set_face_normal(&rotated_r, normal);
            Some(rec)
        } else {
            None
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        if self.has_box {
            Some(self.bbox)
        } else {
            None
        }
    }
}
