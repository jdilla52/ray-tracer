use crate::texture::perlin::Perlin;
use crate::texture::Texture;
use glam::Vec3A;

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
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        Vec3A::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
        // Vec3A::new(1., 1., 1.) * 0.5 * (1.0 + self.noise.noise(p * self.scale))
    }
}
