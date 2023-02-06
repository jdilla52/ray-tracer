use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};

use crate::vec3;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Metal {
    pub texture_index: usize,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(texture_index: usize, fuzz: f32) -> Self {
        Metal {
            texture_index,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected_direction = vec3::reflect(r_in.direction.normalize(), rec.normal);
        let fuzzed_direction = reflected_direction + vec3::random_in_unit_sphere() * self.fuzz;

        if fuzzed_direction.dot(rec.normal) > 0.0 {
            Some(ScatterRecord {
                texture_index: self.texture_index,
                scattered: Ray::new(rec.position, fuzzed_direction, r_in.time),
            })
        } else {
            None
        }
    }
}
