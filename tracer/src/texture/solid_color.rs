use glam::Vec3A;
use crate::texture::Texture;

pub struct SolidColor {
    pub color_value: Vec3A,
}

impl SolidColor {
    pub fn new(color_value: Vec3A) -> Self {
        SolidColor { color_value }
    }

    pub fn new_from_rgb(r: f32, g: f32, b: f32) -> Self {
        SolidColor { color_value: Vec3A::new(r, g, b) }
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A {
        self.color_value
    }
}

