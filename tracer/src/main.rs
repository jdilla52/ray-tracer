mod camera;
mod error;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::hittable::{Hittable, HittableList};
use crate::material::dieletric::Dieletric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use error::TracerResult;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub fn write_image(path: String) -> TracerResult<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let look_from = Vec3::new(3., 3., 2.);
    let look_at = Vec3::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = (look_from - look_at).len();
    let aperture = 2.0;
    let camera = camera::Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let samples = 10;
    let max_depth = 20;

    let r = (std::f64::consts::PI / 4.0).cos();

    let world = HittableList::new(vec![
        Box::new(sphere::Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.3))),
        )),
        Box::new(sphere::Sphere::new(
            Vec3::new(-1.0, 0., -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.2)),
        )),
        Box::new(sphere::Sphere::new(
            Vec3::new(1.0, 0., -1.0),
            0.5,
            Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0)),
        )),
        Box::new(sphere::Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        )),
    ]);

    let mut output = File::create(path)?;
    writeln!(&mut output, "P3\n{} {}\n255", image_width, image_height)?;
    for j in (0..image_height - 1).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let mut color = vec3::Vec3::zero();
            for s in 0..samples {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                color = color + ray_color(&ray, &world, max_depth);
            }
            writeln!(&mut output, "{}", color.as_aggregated_color(samples))?;
        }
    }

    Ok(())
}

pub fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    } else {
        return Some(-half_b - discriminant.sqrt() / a);
    }
}

pub fn ray_color(ray: &ray::Ray, world: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    if let Some(t) = world.hit(ray, 0.001, f64::INFINITY) {
        let target = t.position + t.normal + Vec3::random_unit_vector();
        return if let Some(r) = t.material.scatter(ray, &t) {
            r.attenuation * ray_color(&r.scattered, world, depth - 1)
        } else {
            Vec3::zero()
        };
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::one() * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    write_image("./output/depth_of_field.ppm".to_string()).unwrap();
}
