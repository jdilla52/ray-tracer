use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::texture::Texture;
use crate::vec3;
use glam::Vec3A;
use std::rc::Rc;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + vec3::random_in_unit_sphere().normalize();

        // Catch degenerate scatter direction
        let scatter_direction = if scatter_direction.abs_diff_eq(Vec3A::ZERO, 0.0001) {
            rec.normal
        } else {
            scatter_direction
        };

        Some(ScatterRecord {
            attenuation: self.albedo.value(rec.u, rec.v, rec.position),
            scattered: Ray::new(rec.position, scatter_direction, r_in.time),
        })
    }

    fn color(&self, u: f32, v: f32) -> Vec3A {
        self.albedo.value(u, v, Vec3A::ZERO)
    }
}
