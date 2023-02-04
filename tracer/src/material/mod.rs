pub mod dieletric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use glam::Vec3A;
use crate::material::dieletric::Dieletric;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::isotropic::Isotropic;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;

pub struct ScatterRecord {
    pub attenuation: Vec3A,
    pub scattered: Ray,
}

impl ScatterRecord {
    pub fn new(scattered: Ray, attenuation: Vec3A) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}

// todo move to rc over box - consider once we add threading
// starting to doubt if using pointers to trait objects is the best approach
pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn color(&self, u: f32, v: f32) -> Vec3A {
        Vec3A::ZERO
    }
    fn emitted(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        Vec3A::ZERO
    }
}


pub enum Materials{
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
    fn emitted(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        match self {
            Materials::Lambertian(l) => l.emitted(u, v, p),
            Materials::Metal(m) => m.emitted(u, v, p),
            Materials::Dieletric(d) => d.emitted(u, v, p),
            Materials::Isotropic(i) => i.emitted(u, v, p),
            Materials::DiffuseLight(d) => d.emitted(u, v, p),
        }
    }
}
