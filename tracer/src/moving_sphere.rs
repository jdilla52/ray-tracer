use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use std::rc::Rc;
use glam::Vec3A;

pub struct MovingSphere {
    pub center0: Vec3A,
    pub center1: Vec3A,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(center0: Vec3A, center1: Vec3A, radius: f32, material: Rc<dyn Material>) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3A {
        self.center0 + ((time - 0.0) / (1.0 - 0.0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let position = ray.at(root);
        let outward_normal = (position - self.center(ray.time)) / self.radius;
        if ray.direction.dot(outward_normal) < 0.0 {
            Some(HitRecord {
                root,
                position,
                normal: outward_normal,
                front_face: true,
                material: self.material.clone(),
            })
        } else {
            Some(HitRecord {
                root,
                position,
                normal: -outward_normal,
                front_face: false,
                material: self.material.clone(),
            })
        }
    }
}
