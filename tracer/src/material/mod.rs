pub mod dieletric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::geometry::hittable::HitRecord;
use crate::ray::Ray;
use glam::Vec3A;

pub struct ScatterRecord {
    pub attenuation: Vec3A,
    pub scattered: Ray,
}

impl ScatterRecord {
    pub fn new(scattered: Ray, attenuation: Vec3A) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}

// todo move to rc over box - consider once we add threading
// starting to doubt if using pointers to trait objects is the best approach
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn color(&self, u: f32, v: f32) -> Vec3A;
    fn emitted(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        Vec3A::ZERO
    }
}
//
// trait MaterialClone {
//     fn clone_box(&self) -> Box<dyn Material>;
// }
//
// impl<T> MaterialClone for T
//     where
//         T: 'static + Material + Clone,
// {
//     fn clone_box(&self) -> Box<dyn Material> {
//         Box::new(self.clone())
//     }
// }
//
// impl Clone for Box<dyn Material> {
//     fn clone(&self) -> Box<dyn Material> {
//         self.clone_box()
//     }
// }
