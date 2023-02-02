pub mod checker;
pub mod noise;
pub mod perlin;
pub mod solid;

use glam::Vec3A;

pub trait Texture {
    fn value(&self, u: f32, v: f32, p: Vec3A) -> Vec3A;
}
