use glam::Vec3A;
use std::rc::Rc;
use crate::material::Material;
use crate::intersection::ray::Ray;

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
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3A) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
