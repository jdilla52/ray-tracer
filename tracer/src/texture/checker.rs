use crate::texture::Texture;
use glam::Vec3A;
use std::rc::Rc;

pub struct Checker {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
    pub scale: f32,
}

impl Checker {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>, scale: f32) -> Self {
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
