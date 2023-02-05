use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};
use crate::texture::solid::Solid;
use crate::texture::Texture;
use glam::Vec3A;
use std::rc::Rc;

pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn new_from_color(&self, r: f32, g: f32, b: f32) -> Self {
        Self::new(Rc::new(Solid::new_from_rgb(r, g, b)))
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn color(&self, u: f32, v: f32) -> Vec3A {
        todo!()
    }

    fn emitted(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        self.emit.value(u, v, p)
    }
}
