use crate::error::{TracerError, TracerResult};
use crate::texture::{Texture, Textures};
use glam::Vec3A;
use image;
use image::io::Reader;
use image::{DynamicImage, GenericImageView};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImageBuilder {
    path: String,
}

impl TryInto<Textures> for ImageBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Textures> {
        Ok(Textures::Image(Image::new(&self.path)?))
    }
}

pub struct Image {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
    pub bytes_per_line: u32,
}
const CHANNELS: u32 = 3;
impl Image {
    pub fn new(path: &str) -> TracerResult<Self> {
        let image = Reader::open(path)?.decode()?;
        let (width, height) = image.dimensions();
        // image.get_pixel(0, 0);
        // let image = image.to_rgb32f().into_raw(); // todo use this see below
        Ok(Image {
            image,
            width,
            height,
            bytes_per_line: CHANNELS * width,
        })
    }
}

impl Texture for Image {
    fn value(&self, u: f32, v: f32, _p: Vec3A) -> Vec3A {
        let uu = u.clamp(0.0, 1.0);
        let vv = (-1.0 * v).clamp(0.0, 1.0);
        let i = (uu * self.width as f32) as u32;
        let j = (vv * self.height as f32) as u32;
        let ii = i.min(self.width - 1);
        let jj = j.min(self.height - 1);

        // todo issues with color conversion... maybe needs some type of mapping.
        // let id = (jj * self.bytes_per_line + ii * CHANNELS) as usize;
        // let k = (self.image[id], self.image[id+1], self.image[id+1]);
        // let l = Vec3A::new( self.image[id], self.image[id+1], self.image[id+1]);

        let pixel = self.image.get_pixel(ii, jj);
        Vec3A::new(
            pixel.0[0] as f32 / 255.,
            pixel.0[1] as f32 / 255.,
            pixel.0[2] as f32 / 255.,
        )
    }
}
