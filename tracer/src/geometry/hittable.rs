use crate::error::{TracerError, TracerResult};
use crate::geometry::aabb::Aabb;
use crate::geometry::{Geometry, GeometryFile, Hittable};
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HittableListBuilder {
    pub objects: Vec<GeometryFile>,
}

impl HittableListBuilder {
    pub fn new(objects: Vec<GeometryFile>) -> Self {
        HittableListBuilder { objects }
    }
}

impl TryInto<Geometry> for HittableListBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Geometry> {
        Ok(Geometry::HittableList(HittableList::new(
            self.objects
                .into_iter()
                .map(|object| object.try_into())
                .collect::<TracerResult<Vec<Geometry>>>()?,
        )))
    }
}

pub struct HittableList {
    pub objects: Vec<Geometry>,
}

impl HittableList {
    pub fn new(objects: Vec<Geometry>) -> Self {
        HittableList { objects }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_anything = None;

        // might be interesting to see if we could presort the scene
        // see if we can early out on the first hit
        for object in &self.objects {
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.root;
                hit_anything = Some(hit_record);
            }
        }
        hit_anything
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        if self.objects.is_empty() {
            return None;
        }

        self.objects.iter().fold(None, |output_box, object| {
            if let Some(temp_box) = object.bounding_box(t0, t1) {
                if let Some(output_value) = output_box {
                    Some(Aabb::surrounding_box(&output_value, &temp_box))
                } else {
                    Some(temp_box)
                }
            } else {
                None
            }
        })
    }
}
