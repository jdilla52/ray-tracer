mod camera;
mod error;
mod geometry;
pub mod intersection;
mod material;
mod renderer;
mod texture;
mod vec3;


use std::fs;








use crate::renderer::RenderBuilder;




use error::TracerResult;



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

pub fn render_from_json() -> TracerResult<()> {
    let file = fs::File::open("./assets/test.json")?;
    let render: RenderBuilder = serde_json::from_reader(file)?;
    render.build()?.render()?;
    Ok(())
}

fn main() -> TracerResult<()> {
    render_from_json()
}
