use crate::camera::Camera;
use crate::error::TracerResult;
use crate::geometry::hittable::HittableList;
use crate::geometry::{Geometry, Hittable};
use crate::intersection::ray::Ray;
use crate::material::{Material, Materials};
use crate::texture::{Texture, Textures};
use crate::vec3;
use glam::Vec3A;
use rayon::iter::{ParallelBridge, ParallelIterator};
pub struct RenderSettings {
    pub image_width: u32,
    pub aspect_ratio: f32,
    pub samples: u32,
    pub max_depth: u32,
    pub background_color: Vec3A,
    pub path: String,
}

impl RenderSettings {
    pub fn new(
        image_width: u32,
        aspect_ratio: f32,
        samples: u32,
        max_depth: u32,
        background_color: Vec3A,
        path: String,
    ) -> Self {
        Self {
            image_width,
            aspect_ratio,
            samples,
            max_depth,
            background_color,
            path,
        }
    }
    pub fn image_height(&self) -> u32 {
        (self.image_width as f32 / self.aspect_ratio) as u32
    }
}

pub struct Renderer {
    materials: Vec<Materials>,
    textures: Vec<Textures>,
    geometry: HittableList,
    camera: Camera,

    settings: RenderSettings,
}

impl Renderer {
    pub fn new(
        materials: Vec<Materials>,
        textures: Vec<Textures>,
        geometry: HittableList,
        camera: Camera,
        settings: RenderSettings,
    ) -> Self {
        Self {
            materials,
            textures,
            geometry,
            camera,
            settings,
        }
    }
    pub fn render(&self) -> TracerResult<()> {
        let height = self.settings.image_height();
        let mut imgbuf = image::ImageBuffer::new(self.settings.image_width, height);
        let path = self.settings.path.clone();
        let _rng = rand::thread_rng();
        // todo evaluate if we should change how we're iterating.
        imgbuf
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| self.per_pixel(x, y, pixel, height));
        imgbuf.save(path)?;

        Ok(())
    }

    pub fn per_pixel(&self, x: u32, y: u32, pixel: &mut image::Rgb<u8>, height: u32) {
        let mut color = Vec3A::ZERO;
        for _s in 0..self.settings.samples {
            let u = (x as f32 + rand::random::<f32>()) / (self.settings.image_width - 1) as f32;
            let v = (y as f32 + rand::random::<f32>()) / (height - 1) as f32;
            let v = 1.0 - v; // flip height
            let ray = self.camera.get_ray(u, v);
            let rc = self.ray_color(&ray, self.settings.max_depth as i32);
            // let rc = renderer.ray_color(u, v);
            color += rc;
        }
        let scaled = vec3::sqrt(color * (1.0 / self.settings.samples as f32));
        let clamped = scaled.clamp(Vec3A::ZERO, Vec3A::new(0.999, 0.999, 0.999)) * 255.99;
        *pixel = image::Rgb([clamped.x as u8, clamped.y as u8, clamped.z as u8]);
    }

    // todo add energy conservation
    // pub fn ray_color(
    //     &self,
    //     u: f32,
    //     v: f32,
    // ) -> Vec3A {
    //     let mut ray = self.camera.get_ray(u, v);
    //     let mut s_emitted = Vec3A::ZERO;
    //     let mut multiplier = 1.0;
    //     for i in 0..self.depth {
    //         if let Some(t) = self.geometry.hit(&ray, 0.001, f32::INFINITY) {
    //             let material = &self.materials[t.material_index as usize];
    //             let emitted = material.emitted(t.u, t.v, t.position);
    //             if let Some(r) = material.scatter(&ray, &t) {
    //                 ray = r.scattered;
    //                 // emitted + r.attenuation * self.ray_color(&r.scattered, depth - 1)
    //                 s_emitted += (emitted  * r.attenuation) * multiplier;
    //                 multiplier *= 0.5;
    //             } else {
    //                 // emitted
    //                 return s_emitted * emitted * multiplier;
    //             }
    //         } else {
    //             return s_emitted + self.background_color * multiplier;
    //         }
    //     }
    //     s_emitted
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
            self.settings.background_color
        }
    }
}
