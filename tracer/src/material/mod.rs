pub mod dieletric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;
pub mod pbr;

use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::dieletric::Dieletric;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::isotropic::Isotropic;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use glam::Vec3A;

use serde::{Deserialize, Serialize};
use crate::material::pbr::Pbr;
use crate::texture::TexturesType;

pub struct ScatterRecord {
    pub texture_index: usize,
    pub scattered: Ray,
}

impl ScatterRecord {
    pub fn new(scattered: Ray, texture_index: usize) -> Self {
        Self {
            texture_index,
            scattered,
        }
    }
}

// todo move to rc over box - consider once we add threading
// starting to doubt if using pointers to trait objects is the best approach
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, textures: &Vec<TexturesType>) -> Option<ScatterRecord>;
    fn color(&self, _u: f32, _v: f32) -> Vec3A {
        Vec3A::ZERO
    }
    fn emitted(&self) -> Option<usize> {
        None
    }
}

pub struct MaterialList {
    pub materials: Vec<MaterialType>,
    pub textures: Vec<TexturesType>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dieletric(Dieletric),
    Isotropic(Isotropic),
    DiffuseLight(DiffuseLight),
    Pbr(Pbr),
}

impl Material for MaterialType {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, textures: &Vec<TexturesType>) -> Option<ScatterRecord> {
        match self {
            MaterialType::Lambertian(l) => l.scatter(r_in, rec, textures),
            MaterialType::Metal(m) => m.scatter(r_in, rec, textures),
            MaterialType::Dieletric(d) => d.scatter(r_in, rec, textures),
            MaterialType::Isotropic(i) => i.scatter(r_in, rec, textures),
            MaterialType::DiffuseLight(d) => d.scatter(r_in, rec, textures),
            MaterialType::Pbr(p) => p.scatter(r_in, rec, textures),
        }
    }
    fn color(&self, u: f32, v: f32) -> Vec3A {
        match self {
            MaterialType::Lambertian(l) => l.color(u, v),
            MaterialType::Metal(m) => m.color(u, v),
            MaterialType::Dieletric(d) => d.color(u, v),
            MaterialType::Isotropic(i) => i.color(u, v),
            MaterialType::DiffuseLight(d) => d.color(u, v),
            MaterialType::Pbr(p) => p.color(u, v),
        }
    }
    fn emitted(&self) -> Option<usize> {
        match self {
            MaterialType::Lambertian(l) => l.emitted(),
            MaterialType::Metal(m) => m.emitted(),
            MaterialType::Dieletric(d) => d.emitted(),
            MaterialType::Isotropic(i) => i.emitted(),
            MaterialType::DiffuseLight(d) => d.emitted(),
            MaterialType::Pbr(p) => p.emitted(),
        }
    }
}
