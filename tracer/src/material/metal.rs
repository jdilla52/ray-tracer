use crate::geometry::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3;
use glam::Vec3A;
use std::rc::Rc;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Rc<dyn Texture>,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Rc<dyn Texture>, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected_direction = vec3::reflect(r_in.direction.normalize(), rec.normal);
        let fuzzed_direction = reflected_direction + vec3::random_in_unit_sphere() * self.fuzz;

        if fuzzed_direction.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo.value(rec.u, rec.v, rec.position),
                scattered: Ray::new(rec.position, fuzzed_direction, r_in.time),
            })
        } else {
            None
        }
    }

    fn color(&self, u: f32, v: f32) -> Vec3A {
        self.albedo.value(u, v, Vec3A::ZERO)
    }
}
