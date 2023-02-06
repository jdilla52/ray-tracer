use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};

use crate::vec3::random_in_unit_sphere;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Isotropic {
    pub texture_index: usize,
}

impl Isotropic {
    pub fn new(texture_index: usize) -> Self {
        Self { texture_index }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        Some(ScatterRecord::new(
            Ray::new(rec.position, random_in_unit_sphere(), r_in.time),
            self.texture_index,
        ))
    }
}
