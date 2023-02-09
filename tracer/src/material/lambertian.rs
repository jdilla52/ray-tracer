use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::{Material, ScatterRecord};
use crate::texture::Texture;
use crate::vec3;
use glam::Vec3A;


#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Box<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Box<dyn Texture>) -> Self {
        Lambertian { albedo }
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
            attenuation: self.albedo.value(rec.u, rec.v, rec.position),
            scattered: Ray::new(rec.position, scatter_direction, r_in.time),
        })
    }

    fn color(&self, u: f32, v: f32) -> Vec3A {
        self.albedo.value(u, v, Vec3A::ZERO)
    }
}
