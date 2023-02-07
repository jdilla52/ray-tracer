use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};

use crate::vec3;

use serde::{Deserialize, Serialize};
use crate::texture::TexturesType;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dieletric {
    pub ref_idx: f32,
    pub texture_index: usize,
    pub emitted: Option<usize>,
}

impl Dieletric {
    pub fn new(ref_idx: f32, texture_index: usize, emitted: Option<usize>) -> Self {
        Dieletric {
            ref_idx,
            texture_index,
            emitted
        }
    }
    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, textures: &Vec<TexturesType>) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(r_in.direction.normalize(), rec.normal);
        // let attenuation = Vec3A::new(1.0, 1.0, 1.0);

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
            texture_index: self.texture_index,
            scattered: Ray::new(rec.position, direction, r_in.time),
        })
    }
    fn emitted(&self) -> Option<usize> {
        if let Some(emitted) = self.emitted {
            Some(emitted)
        } else {
            None
        }
    }
}
