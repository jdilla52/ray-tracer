use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiffuseLight {
    texture_index: usize,
}

impl DiffuseLight {
    pub fn new(texture_index: usize) -> Self {
        Self { texture_index }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self) -> Option<usize> {
        Some(self.texture_index)
    }
}
