use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;
use std::rc::Rc;

pub struct HitRecord {
    pub root: f32,
    pub position: Vec3A,
    pub normal: Vec3A,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {}

// enum Hittable

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        HittableList { objects }
    }

    pub fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // let mut hit_anything = None;
        // let mut closest_so_far = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(r, t_min, t_max) {
                return Some(hit_record);
                // closest_so_far = hit_record.root;
                // hit_anything = Some(hit_record);
            }
        }
        // hit_anything
        None
    }
}
