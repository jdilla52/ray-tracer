use crate::camera::Camera;
use crate::geometry::hittable::HittableList;
use crate::geometry::{Geometry, Hittable};
use crate::intersection::ray::Ray;
use crate::material::{Material, Materials};
use crate::texture::{Texture, Textures};
use glam::Vec3A;

pub struct Renderer {
    materials: Vec<Materials>,
    textures: Vec<Textures>,
    geometry: HittableList,
    camera: Camera,

    background_color: Vec3A,
    depth: i32,
}

impl Renderer {
    pub fn new(
        materials: Vec<Materials>,
        textures: Vec<Textures>,
        geometry: HittableList,
        camera: Camera,
        background_color: Vec3A,
        depth: i32,
    ) -> Self {
        Self {
            materials,
            textures,
            geometry,
            camera,
            background_color,
            depth,
        }
    }

    // pub fn ray_color(
    //     &self,
    //     u: f32,
    //     v: f32,
    // ) -> Vec3A {
    //     let ray = self.camera.get_ray(u, v);
    //     let mut color = Vec3A::new(0.0, 0.0, 0.0);
    //     for i in 0..self.depth {
    //         if let Some(t) = self.geometry.hit(&ray, 0.001, f32::INFINITY) {
    //             let material = &self.materials[t.material_index as usize];
    //             let emitted = material.emitted(t.u, t.v, t.position);
    //             if let Some(r) = material.scatter(&ray, &t) {
    //                 // emitted + r.attenuation * self.ray_color(&r.scattered, depth - 1)
    //                 color = emitted + r.attenuation * color;
    //             } else {
    //                 // emitted
    //                 return color + emitted
    //             }
    //         } else {
    //             return color + self.background_color;
    //         }
    //     }
    //     color
    // }

    pub fn ray_color(&self, ray: &Ray, depth: i32) -> Vec3A {
        if let Some(t) = self.geometry.hit(ray, 0.001, f32::INFINITY) {
            let material = &self.materials[t.material_index as usize];
            let emitted = material.emitted(t.u, t.v, t.position);
            if let Some(r) = material.scatter(ray, &t) {
                emitted + r.attenuation * self.ray_color(&r.scattered, depth - 1)
            } else {
                emitted
            }
        } else {
            self.background_color
        }
    }
}
