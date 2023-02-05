use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};
use crate::texture::solid::Solid;
use crate::texture::Texture;
use crate::vec3::random_in_unit_sphere;
use glam::Vec3A;
use std::rc::Rc;

pub struct Isotropic {
    pub albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }
    pub fn new_color(albedo: Vec3A) -> Self {
        Self {
            albedo: Rc::new(Solid::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new(
            Ray::new(rec.position, random_in_unit_sphere(), r_in.time),
            self.albedo.value(rec.u, rec.v, rec.position),
        ))
    }
    fn color(&self, u: f32, v: f32) -> Vec3A {
        self.albedo.value(u, v, Vec3A::ZERO)
    }
}
