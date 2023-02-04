use crate::geometry::aabb::Aabb;
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
    pub u: f32,
    pub v: f32,
}

impl HitRecord {
    pub fn new(
        root: f32,
        ray: &Ray,
        outward_normal: Vec3A,
        material: Rc<dyn Material>,
        u: f32,
        v: f32,
    ) -> Self {
        let position = ray.at(root);
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            root,
            position,
            normal,
            front_face,
            material,
            u,
            v,
        }
    }
}

// enum Hittable

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> Self {
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

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
