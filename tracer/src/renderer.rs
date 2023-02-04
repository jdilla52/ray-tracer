use crate::geometry::{Geometry, Hittable};
use crate::material::{Material, Materials};
use crate::texture::{Texture, Textures};

pub struct Renderer {
    materials: Vec<Materials>,
    textures: Vec<Textures>,
    geometry: Vec<Geometry>,
}