use crate::camera::{CamerBuilder, Camera};
use crate::error::TracerResult;
use crate::geometry::hittable::{HittableListBuilder};
use crate::geometry::{Geometry, Hittable};
use crate::intersection::ray::Ray;
use crate::material::{Material, Materials};
use crate::texture::{Texture, TextureFile, Textures};
use crate::vec3;
use glam::Vec3A;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RenderBuilder {
    pub settings: RenderSettings,
    pub world: HittableListBuilder,
    pub camera: CamerBuilder,
    pub materials: Vec<Materials>,
    pub textures: Vec<TextureFile>,
}

impl RenderBuilder {
    pub fn build(self) -> TracerResult<Renderer> {
        let textures = self
            .textures
            .into_iter()
            .map(|t| t.try_into())
            .collect::<TracerResult<Vec<Textures>>>()?;
        let camera = self.camera.build();
        let geometry = self.world.try_into()?;
        let settings = self.settings.clone();

        Ok(Renderer::new(
            self.materials.clone(),
            textures,
            geometry,
            camera,
            settings,
        ))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
    geometry: Geometry,
    camera: Camera,

    settings: RenderSettings,
}

impl Renderer {
    pub fn new(
        materials: Vec<Materials>,
        textures: Vec<Textures>,
        geometry: Geometry,
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
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let mut color = Vec3A::ZERO;
            for _s in 0..self.settings.samples {
                let u = (x as f32 + rand::random::<f32>()) / (self.settings.image_width - 1) as f32;
                let v = (y as f32 + rand::random::<f32>()) / (height - 1) as f32;
                let ray = self.camera.get_ray(u, v);
                let rc = self.ray_color(&ray, self.settings.max_depth as i32);
                // let rc = renderer.ray_color(u, v);
                color += rc;
            }
            let scaled = vec3::sqrt(color * (1.0 / self.settings.samples as f32));
            let clamped = scaled.clamp(Vec3A::ZERO, Vec3A::new(0.999, 0.999, 0.999)) * 255.99;
            *pixel = image::Rgb([clamped.x as u8, clamped.y as u8, clamped.z as u8]);
        }
        imgbuf.save(path)?;

        Ok(())
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
            let emitted = if let Some(id) = material.emitted() {
                self.textures[id].value(t.u, t.v, t.position)
            } else {
                Vec3A::ZERO
            };
            if let Some(r) = material.scatter(ray, &t) {
                let attenuation = self.textures[r.texture_index].value(t.u, t.v, t.position);
                emitted + attenuation * self.ray_color(&r.scattered, depth - 1)
            } else {
                emitted
            }
        } else {
            self.settings.background_color
        }
    }
}
