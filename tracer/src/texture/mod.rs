pub mod checker;
pub mod image;
pub mod noise;
pub mod perlin;
pub mod solid;

use crate::texture::checker::Checker;
use crate::texture::image::Image;
use crate::texture::noise::Noise;
use crate::texture::solid::Solid;
use glam::Vec3A;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A;
}

pub enum Textures {
    Checker(Checker),
    Image(Image),
    Noise(Noise),
    Solid(Solid),
}

impl Texture for Textures {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        match self {
            Textures::Checker(t) => t.value(u, v, p),
            Textures::Image(t) => t.value(u, v, p),
            Textures::Noise(t) => t.value(u, v, p),
            Textures::Solid(t) => t.value(u, v, p),
        }
    }
}
