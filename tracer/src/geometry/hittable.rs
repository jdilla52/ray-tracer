use crate::geometry::aabb::Aabb;
use crate::material::Material;
use crate::intersection::ray::Ray;
use glam::Vec3A;
use std::rc::Rc;
use crate::geometry::Hittable;
use crate::intersection::hit_record::HitRecord;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> Self {
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
