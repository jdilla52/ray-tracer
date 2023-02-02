use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3;
use glam::Vec3A;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3A,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3A, fuzz: f32) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = vec3::reflect(r_in.direction.normalize(), rec.normal);
        let fuzzed_direction = scatter_direction + vec3::random_in_unit_sphere() * self.fuzz;

        if fuzzed_direction.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: Ray::new(rec.position, fuzzed_direction),
            })
        } else {
            None
        }
    }

    fn color(&self) -> Vec3A {
        self.albedo
    }
}
