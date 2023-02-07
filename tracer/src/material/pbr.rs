use glam::Vec3A;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};
use crate::texture::{Texture, TexturesType};
use crate::vec3;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Pbr {
    texture_index: usize, // albedo
    roughness_index: usize,
    emitted: Option<usize>,
    // metalness: usize,
    // ao: usize,
}

impl Pbr {
    pub fn new(texture_index: usize, roughness_index: usize, emitted: Option<usize>) -> Self {
        Self {
            texture_index,
            roughness_index,
            emitted
            // metalness,
        }
    }
}

impl Material for Pbr {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, textures: &Vec<TexturesType>) -> Option<ScatterRecord> {

        let roughness = textures[self.roughness_index].value(rec.u, rec.v, rec.position);
        // let color = self.albedo.value(rec.u, rec.v, rec.position);
        // let metalness = self.metalness.value(rec.u, rec.v, rec.position);
        // todo check if we need to validate for degenerate cases

        // reflect based on roughness
        let scatter_direction = vec3::reflect(r_in.direction, rec.normal + roughness * vec3::random_in_unit_sphere().normalize());
        Some(ScatterRecord {
            texture_index: self.texture_index,
            scattered: Ray::new(rec.position, scatter_direction, r_in.time),
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