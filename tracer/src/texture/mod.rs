pub mod checker;
pub mod image;
pub mod noise;
pub mod perlin;
pub mod solid;

use std::fmt::{Debug, Formatter};
use crate::texture::checker::Checker;
use crate::texture::image::Image;
use crate::texture::noise::Noise;
use crate::texture::solid::Solid;
use glam::Vec3A;

pub trait Texture: Send + Sync + CloneTexture + Debug {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A;
}

#[derive(Debug, Clone)]
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

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.clone_dyn()
    }
}

pub trait CloneTexture {
    fn clone_dyn<'a>(&self) -> Box<dyn Texture>;
}

impl<T> CloneTexture for T
    where
        T: Texture + Clone + 'static,
{
    fn clone_dyn(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}