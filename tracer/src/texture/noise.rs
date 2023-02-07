use crate::texture::perlin::Perlin;
use crate::texture::{Texture, TexturesType};
use glam::Vec3A;

use crate::error::{TracerError, TracerResult};
use serde::{Deserialize, Serialize};

fn default_scale() -> f32 {
    256.0
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoiseBuilder {
    #[serde(default = "default_scale")]
    scale: f32,
}

impl NoiseBuilder {
    pub fn new(scale: f32) -> Self {
        NoiseBuilder { scale }
    }
}

impl TryInto<TexturesType> for NoiseBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<TexturesType> {
        Ok(TexturesType::Noise(Noise::new(self.scale)))
    }
}

pub struct Noise {
    pub scale: f32,
    pub noise: Perlin,
}

impl Noise {
    pub fn new(scale: f32) -> Self {
        Noise {
            scale,
            noise: Perlin::new(),
        }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, p: Vec3A) -> Vec3A {
        Vec3A::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
        // Vec3A::new(1., 1., 1.) * 0.5 * (1.0 + self.noise.noise(p * self.scale))
    }
}
