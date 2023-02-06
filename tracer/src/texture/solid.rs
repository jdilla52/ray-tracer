use crate::texture::Texture;
use glam::Vec3A;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Solid {
    pub color_value: Vec3A,
}

impl Solid {
    pub fn new(color_value: Vec3A) -> Self {
        Solid { color_value }
    }

    pub fn new_from_rgb(r: f32, g: f32, b: f32) -> Self {
        Solid {
            color_value: Vec3A::new(r, g, b),
        }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f32, _v: f32, _p: Vec3A) -> Vec3A {
        self.color_value
    }
}
