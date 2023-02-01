use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = r_in.direction.unit().reflect(&rec.normal);

        if scatter_direction.dot(&rec.normal) > 0.0 {
            Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: Ray::new(rec.position, scatter_direction),
            })
        } else {
            None
        }
    }

    fn color(&self) -> Vec3 {
        self.albedo
    }
}
