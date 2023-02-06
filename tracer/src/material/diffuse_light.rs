use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};
use crate::texture::solid::Solid;
use crate::texture::Texture;
use glam::Vec3A;
use std::rc::Rc;

pub struct DiffuseLight {
    texture_index: usize,
}

impl DiffuseLight {
    pub fn new(texture_index: usize) -> Self {
        Self { texture_index }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self) -> Option<usize> {
        Some(self.texture_index)
    }
}
