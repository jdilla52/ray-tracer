mod camera;
mod error;
mod geometry;
pub mod intersection;
mod material;
mod renderer;
mod texture;
mod vec3;

use crate::geometry::hittable::{HittableListBuilder};
use std::fs;


use crate::geometry::sphere::Sphere;

use crate::geometry::{GeometryFile};

use crate::material::lambertian::Lambertian;

use crate::material::Materials;
use crate::renderer::{RenderBuilder};

use crate::texture::image::{ImageBuilder};

use crate::texture::{TextureFile};
use error::TracerResult;


use glam::Vec3A;

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

fn earth() -> (Vec<Materials>, Vec<TextureFile>, HittableListBuilder) {
    let textures = vec![TextureFile::Image(ImageBuilder::new(
        "./assets/earthmap.jpg".to_string(),
    ))];
    let materials = vec![Materials::Lambertian(Lambertian::new(0))];
    let geo = HittableListBuilder::new(vec![GeometryFile::Sphere(Sphere::new(
        Vec3A::new(0.0, 0.0, 0.0),
        2.0,
        0,
    ))]);

    (materials, textures, geo)
}
pub fn write_image() -> TracerResult<()> {
    let file = fs::File::open("./assets/test.json")?;
    let render: RenderBuilder = serde_json::from_reader(file)?;
    render.build()?.render()?;

    // let (mat, texture, geo) = earth();
    //
    // let settings =  RenderSettings {
    //     image_width: 400,
    //     aspect_ratio: 16.0 / 9.0,
    //     samples: 100,
    //     max_depth: 10,
    //     background_color: Vec3A::new(0.0, 0.0, 0.0),
    //     path,
    // };
    // let look_from = Vec3A::new(13., 2., 3.);
    // let look_at = Vec3A::new(0., 0., 0.);
    // let camera = CamerBuilder::new(
    //    look_from,
    //     look_at,
    //     Vec3A::new(0., 1., 0.),
    //     20.,
    //     16.0 / 9.0,
    //     2.0,
    //     (look_from - look_at).length(),
    //     0.0,
    //     1.0,
    // );
    //
    //
    // let r = RenderBuilder{
    //     settings,
    //     world: geo,
    //     camera,
    //     materials: mat,
    //     textures: texture,
    // };
    //
    // serde_json::to_writer_pretty(File::create("./output/test.json").unwrap(), &r).unwrap();

    // let renderer = Renderer::new(
    //     mat,
    //     texture,
    //     geo,
    //     camera.clone(),
    //     settings
    // );
    //
    // renderer.render()?;

    Ok(())
}

fn main() -> TracerResult<()> {
    write_image()
}
