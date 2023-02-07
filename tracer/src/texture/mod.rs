pub mod checker;
pub mod image;
pub mod noise;
pub mod perlin;
pub mod solid;

use crate::texture::checker::{Checker, CheckerBuilder};
use crate::texture::image::{Image, ImageBuilder};
use crate::texture::noise::{Noise, NoiseBuilder};
use crate::texture::solid::Solid;
use glam::Vec3A;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A;
}
use crate::error::{TracerError, TracerResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TextureFile {
    Checker(CheckerBuilder),
    Image(ImageBuilder),
    Noise(NoiseBuilder),
    Solid(Solid),
}

impl TryInto<TexturesType> for TextureFile {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<TexturesType> {
        match self {
            TextureFile::Checker(c) => Ok(c.try_into()?),
            TextureFile::Image(i) => Ok(i.try_into()?),
            TextureFile::Noise(n) => Ok(n.try_into()?),
            TextureFile::Solid(s) => Ok(TexturesType::Solid(s)),
        }
    }
}

impl TryInto<Box<TexturesType>> for Box<TextureFile> {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Box<TexturesType>> {
        match *self {
            TextureFile::Checker(c) => Ok(Box::new(c.try_into()?)),
            TextureFile::Image(i) => Ok(Box::new(i.try_into()?)),
            TextureFile::Noise(n) => Ok(Box::new(n.try_into()?)),
            TextureFile::Solid(s) => Ok(Box::new(TexturesType::Solid(s))),
        }
    }
}

struct TextureType {
    textures: Vec<TextureType>,
}

pub enum TexturesType {
    Checker(Checker),
    Image(Image),
    Noise(Noise),
    Solid(Solid),
}

impl Texture for TexturesType {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        match self {
            TexturesType::Checker(t) => t.value(u, v, p),
            TexturesType::Image(t) => t.value(u, v, p),
            TexturesType::Noise(t) => t.value(u, v, p),
            TexturesType::Solid(t) => t.value(u, v, p),
        }
    }
}
