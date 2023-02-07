use crate::texture::{Texture, TextureFile, TexturesType};
use glam::Vec3A;
use std::boxed::Box;

use crate::error::{TracerError, TracerResult};
use serde::{Deserialize, Serialize};

fn default_scale() -> f32 {
    10.0
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckerBuilder {
    pub odd: Box<TextureFile>,
    pub even: Box<TextureFile>,
    #[serde(default = "default_scale")]
    pub scale: f32,
}

impl CheckerBuilder {
    pub fn new(odd: Box<TextureFile>, even: Box<TextureFile>, scale: f32) -> Self {
        CheckerBuilder { odd, even, scale }
    }
}

impl TryInto<TexturesType> for CheckerBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<TexturesType> {
        Ok(TexturesType::Checker(Checker::new(
            self.odd.try_into()?,
            self.even.try_into()?,
            self.scale,
        )))
    }
}

pub struct Checker {
    pub odd: Box<TexturesType>,
    pub even: Box<TexturesType>,
    pub scale: f32,
}

impl Checker {
    pub fn new(odd: Box<TexturesType>, even: Box<TexturesType>, scale: f32) -> Self {
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
