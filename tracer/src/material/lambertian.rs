use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3;
use glam::Vec3A;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3A,
}

impl Lambertian {
    pub fn new(albedo: Vec3A) -> Self {
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
            attenuation: self.albedo,
            scattered: Ray::new(rec.position, scatter_direction),
        })
    }

    fn color(&self) -> Vec3A {
        self.albedo
    }
}
