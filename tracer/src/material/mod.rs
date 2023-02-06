pub mod dieletric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::dieletric::Dieletric;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::isotropic::Isotropic;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use glam::Vec3A;

use serde::{Deserialize, Serialize};

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn color(&self, _u: f32, _v: f32) -> Vec3A {
        Vec3A::ZERO
    }
    fn emitted(&self) -> Option<usize> {
        None
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Materials {
    Lambertian(Lambertian),
    Metal(Metal),
    Dieletric(Dieletric),
    Isotropic(Isotropic),
    DiffuseLight(DiffuseLight),
}

impl Material for Materials {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Materials::Lambertian(l) => l.scatter(r_in, rec),
            Materials::Metal(m) => m.scatter(r_in, rec),
            Materials::Dieletric(d) => d.scatter(r_in, rec),
            Materials::Isotropic(i) => i.scatter(r_in, rec),
            Materials::DiffuseLight(d) => d.scatter(r_in, rec),
        }
    }
    fn color(&self, u: f32, v: f32) -> Vec3A {
        match self {
            Materials::Lambertian(l) => l.color(u, v),
            Materials::Metal(m) => m.color(u, v),
            Materials::Dieletric(d) => d.color(u, v),
            Materials::Isotropic(i) => i.color(u, v),
            Materials::DiffuseLight(d) => d.color(u, v),
        }
    }
    fn emitted(&self) -> Option<usize> {
        match self {
            Materials::Lambertian(l) => l.emitted(),
            Materials::Metal(m) => m.emitted(),
            Materials::Dieletric(d) => d.emitted(),
            Materials::Isotropic(i) => i.emitted(),
            Materials::DiffuseLight(d) => d.emitted(),
        }
    }
}
