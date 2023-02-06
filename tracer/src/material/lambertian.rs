use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};

use crate::vec3;
use glam::Vec3A;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Lambertian {
    pub texture_index: usize,
}

impl Lambertian {
    pub fn new(texture_index: usize) -> Self {
        Lambertian { texture_index }
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
            texture_index: self.texture_index,
            scattered: Ray::new(rec.position, scatter_direction, r_in.time),
        })
    }
}
