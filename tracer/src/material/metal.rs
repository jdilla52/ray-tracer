use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = r_in.direction.unit().reflect(&rec.normal);
        let fuzzed_direction = scatter_direction + Vec3::random_in_unit_sphere() * self.fuzz;

        if fuzzed_direction.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: Ray::new(rec.position, fuzzed_direction),
            })
        } else {
            None
        }
    }

    fn color(&self) -> Vec3 {
        self.albedo
    }
}
