use crate::texture::{Texture, TextureFile, Textures};
use glam::Vec3A;
use std::boxed::Box;

use crate::error::{TracerError, TracerResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckerBuilder {
    pub odd: Box<TextureFile>,
    pub even: Box<TextureFile>,
    pub scale: f32,
}

impl CheckerBuilder {
    pub fn new(odd: Box<TextureFile>, even: Box<TextureFile>, scale: f32) -> Self {
        CheckerBuilder { odd, even, scale }
    }
}

impl TryInto<Textures> for CheckerBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Textures> {
        Ok(Textures::Checker(Checker::new(
            self.odd.try_into()?,
            self.even.try_into()?,
            self.scale,
        )))
    }
}

pub struct Checker {
    pub odd: Box<Textures>,
    pub even: Box<Textures>,
    pub scale: f32,
}

impl Checker {
    pub fn new(odd: Box<Textures>, even: Box<Textures>, scale: f32) -> Self {
        Checker { odd, even, scale }
    }
}

impl Texture for Checker {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        let sines = (p.x * self.scale).sin() * (p.y * self.scale).sin() * (p.z * self.scale).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
