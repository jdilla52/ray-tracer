pub mod dieletric;
pub mod lambertian;
pub mod metal;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

// todo move to rc over box - consider once we add threading
// starting to doubt if using pointers to trait objects is the best approach
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn color(&self) -> Vec3;
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
