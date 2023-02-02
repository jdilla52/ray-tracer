use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray;
use crate::vec3;
use glam::Vec3A;

pub struct Dieletric {
    pub ref_idx: f32,
}

impl Dieletric {
    pub fn new(ref_idx: f32) -> Self {
        Dieletric { ref_idx }
    }
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(r_in.direction.normalize(), rec.normal);
        let attenuation = Vec3A::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > rand::random::<f32>()
        {
            reflected
        } else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        Some(ScatterRecord {
            attenuation,
            scattered: Ray::new(rec.position, direction, r_in.time),
        })
    }

    fn color(&self) -> Vec3A {
        Vec3A::ZERO
    }
}
