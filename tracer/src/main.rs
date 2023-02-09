mod camera;
mod error;
mod geometry;
pub mod intersection;
mod material;
mod renderer;
mod texture;
mod vec3;

use crate::geometry::bvh::BvhNode;
use crate::geometry::constant_medium::ConstantMedium;
use crate::geometry::cornell_box::CornellBox;
use crate::geometry::hittable::HittableList;
use crate::geometry::rotate_y::RotateY;
use crate::geometry::sphere::Sphere;
use crate::geometry::translate::Translate;
use crate::geometry::xy_rect::XyRect;
use crate::geometry::xz_rect::XzRect;
use crate::geometry::yz_rect::YzRect;
use crate::geometry::Geometry;
use crate::intersection::ray::Ray;
use crate::material::dieletric::Dieletric;
use crate::material::diffuse_light::DiffuseLight;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::{Material, Materials};
use crate::renderer::{RenderSettings, Renderer};
use crate::texture::checker::Checker;
use crate::texture::image::Image;
use crate::texture::noise::Noise;
use crate::texture::solid::Solid;
use error::TracerResult;
use geometry::Hittable;
use glam::Vec3A;
use image::RgbImage;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
//
// fn simple_light() -> HittableList {
//     let noise = Rc::new(Noise::new(4.0));
//     HittableList::new(vec![
//         Rc::new(Sphere::new(
//             Vec3A::new(0.0, -1000.0, 0.0),
//             1000.0,
//             Rc::new(Lambertian::new(noise.clone())),
//         )),
//         Rc::new(Sphere::new(
//             Vec3A::new(0.0, 2.0, 0.0),
//             2.0,
//             Rc::new(Lambertian::new(noise.clone())),
//         )),
//         Rc::new(XyRect::new(
//             3.0,
//             5.0,
//             1.0,
//             3.0,
//             -2.0,
//             Rc::new(DiffuseLight::new(Rc::new(Solid::new(Vec3A::new(
//                 4.0, 4.0, 4.0,
//             ))))),
//         )),
//     ])
// }
//
// fn empty_box() -> HittableList {
//     let white = Rc::new(Lambertian::new(Rc::new(Solid::new(Vec3A::new(
//         0.73, 0.73, 0.73,
//     )))));
//
//     HittableList::new(vec![
//         Rc::new(YzRect::new(
//             0.0,
//             555.0,
//             0.0,
//             555.0,
//             555.0,
//             Rc::new(Lambertian::new(Rc::new(Solid::new(Vec3A::new(
//                 0.12, 0.45, 0.15,
//             ))))),
//         )),
//         Rc::new(YzRect::new(
//             0.0,
//             555.0,
//             0.0,
//             555.0,
//             0.0,
//             Rc::new(Lambertian::new(Rc::new(Solid::new(Vec3A::new(
//                 0.65, 0.05, 0.05,
//             ))))),
//         )),
//         Rc::new(XzRect::new(
//             113.,
//             443.,
//             127.,
//             432.,
//             554.,
//             Rc::new(DiffuseLight::new(Rc::new(Solid::new(Vec3A::new(
//                 15.0, 15.0, 15.0,
//             ))))),
//         )),
//         Rc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white.clone())),
//         Rc::new(XzRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white.clone())),
//         Rc::new(XyRect::new(0.0, 555.0, 0.0, 555.0, 555., white.clone())),
//     ])
// }
//
// fn two_boxes() -> HittableList {
//     let room = empty_box();
//
//     let white = Rc::new(Lambertian::new(Rc::new(Solid::new(Vec3A::new(
//         0.73, 0.73, 0.73,
//     )))));
//
//     HittableList::new(vec![
//         Rc::new(CornellBox::new(
//             Vec3A::new(130., 0.0, 65.0),
//             Vec3A::new(295.0, 165.0, 230.0),
//             white.clone(),
//         )),
//         Rc::new(CornellBox::new(
//             Vec3A::new(265., 0.0, 295.0),
//             Vec3A::new(430.0, 330.0, 460.0),
//             white.clone(),
//         )),
//         Rc::new(room),
//     ])
// }
//
// fn rotate_box() -> HittableList {
//     let room = empty_box();
//     let white = Rc::new(Lambertian::new(Rc::new(Solid::new(Vec3A::new(
//         0.73, 0.73, 0.73,
//     )))));
//     HittableList::new(vec![
//         Rc::new(Translate::new(
//             Rc::new(RotateY::new(
//                 Rc::new(CornellBox::new(
//                     Vec3A::new(0.0, 0.0, 0.0),
//                     Vec3A::new(165.0, 165.0, 165.0),
//                     white.clone(),
//                 )),
//                 -18.0,
//             )),
//             Vec3A::new(130.0, 0.0, 65.0),
//         )),
//         Rc::new(Translate::new(
//             Rc::new(RotateY::new(
//                 Rc::new(CornellBox::new(
//                     Vec3A::new(0.0, 0.0, 0.0),
//                     Vec3A::new(165.0, 330.0, 165.0),
//                     white.clone(),
//                 )),
//                 15.0,
//             )),
//             Vec3A::new(265.0, 0.0, 295.0),
//         )),
//         Rc::new(room),
//     ])
// }
//
// fn rotate_cloudy_box() -> HittableList {
//     let room = empty_box();
//     let white = Rc::new(Lambertian::new(Rc::new(Solid::new(Vec3A::new(
//         0.73, 0.73, 0.73,
//     )))));
//     HittableList::new(vec![
//         Rc::new(ConstantMedium::new_from_density(
//             Rc::new(Translate::new(
//                 Rc::new(RotateY::new(
//                     Rc::new(CornellBox::new(
//                         Vec3A::new(0.0, 0.0, 0.0),
//                         Vec3A::new(165.0, 165.0, 165.0),
//                         white.clone(),
//                     )),
//                     -18.0,
//                 )),
//                 Vec3A::new(130.0, 0.0, 65.0),
//             )),
//             0.01,
//             Vec3A::ZERO,
//         )),
//         Rc::new(ConstantMedium::new_from_density(
//             Rc::new(Translate::new(
//                 Rc::new(RotateY::new(
//                     Rc::new(CornellBox::new(
//                         Vec3A::new(0.0, 0.0, 0.0),
//                         Vec3A::new(165.0, 330.0, 165.0),
//                         white.clone(),
//                     )),
//                     15.0,
//                 )),
//                 Vec3A::new(265.0, 0.0, 295.0),
//             )),
//             0.01,
//             Vec3A::ZERO,
//         )),
//         Rc::new(room),
//     ])
// }

fn earth() -> (Vec<Materials>, HittableList) {
    let materials = vec![Materials::Lambertian(Lambertian::new(Box::new(
        Image::new("./assets/earthmap.jpg").unwrap(),
    )))];
    let geo = HittableList::new(vec![Geometry::Sphere(Sphere::new(
        Vec3A::new(0.0, 0.0, 0.0),
        2.0,
        0,
    ))]);

    (materials, geo)
}
pub fn write_image(path: String) -> TracerResult<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let look_from = Vec3A::new(13., 2., 3.);
    let look_at = Vec3A::new(0., 0., 0.);
    let vup = Vec3A::new(0., 1., 0.);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let background = &Vec3A::new(1.0, 1.0, 1.0);
    let camera = camera::Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    let samples = 100;
    let max_depth = 10;

    let (mat, geo) = earth();

    let renderer = Renderer::new(
        mat,
        vec![],
        geo,
        camera.clone(),
        RenderSettings {
            image_width,
            aspect_ratio,
            samples,
            max_depth,
            background_color: background.clone(),
            path,
        },
    );

    renderer.render()?;

    Ok(())
}

fn main() {
    write_image("./output/test.png".to_string()).unwrap();
}
