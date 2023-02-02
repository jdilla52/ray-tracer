mod camera;
mod error;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;
mod moving_sphere;

use crate::hittable::{Hittable, HittableList};
use crate::material::dieletric::Dieletric;
use crate::material::lambertian::Lambertian;
use crate::material::metal::Metal;
use crate::material::Material;
use crate::ray::Ray;
use error::TracerResult;
use glam::Vec3A;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

pub fn write_image(path: String) -> TracerResult<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let look_from = Vec3A::new(3., 3., 2.);
    let look_at = Vec3A::new(0., 0., -1.);
    let vup = Vec3A::new(0., 1., 0.);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let camera = camera::Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0

    );

    let samples = 10;
    let max_depth = 20;

    let r = (std::f32::consts::PI / 4.0).cos();

    let world = HittableList::new(vec![
        Box::new(sphere::Sphere::new(
            Vec3A::new(0.0, 0.0, -1.0),
            0.5,
            Rc::new(Lambertian::new(Vec3A::new(0.1, 0.2, 0.3))),
        )),
        Box::new(sphere::Sphere::new(
            Vec3A::new(-1.0, 0., -1.0),
            0.5,
            Rc::new(Metal::new(Vec3A::new(0.8, 0.8, 0.8), 0.2)),
        )),
        Box::new(sphere::Sphere::new(
            Vec3A::new(1.0, 0., -1.0),
            0.5,
            Rc::new(Metal::new(Vec3A::new(0.8, 0.6, 0.2), 1.0)),
        )),
        Box::new(sphere::Sphere::new(
            Vec3A::new(0.0, -100.5, -1.0),
            100.0,
            Rc::new(Lambertian::new(Vec3A::new(0.8, 0.8, 0.0))),
        )),
    ]);

    let mut output = File::create(path)?;
    writeln!(&mut output, "P3\n{} {}\n255", image_width, image_height)?;
    for j in (0..image_height - 1).rev() {
        println!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f32 / (image_width - 1) as f32;
            let v = j as f32 / (image_height - 1) as f32;

            let mut color = Vec3A::ZERO;
            for s in 0..samples {
                let u = (i as f32 + rand::random::<f32>()) / (image_width - 1) as f32;
                let v = (j as f32 + rand::random::<f32>()) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                color = color + ray_color(&ray, &world, max_depth);
            }
            writeln!(&mut output, "{}", vec3::as_aggregated_color(color, samples))?;
        }
    }

    Ok(())
}

pub fn hit_sphere(center: Vec3A, radius: f32, ray: &Ray) -> Option<f32> {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return None;
    } else {
        return Some(-half_b - discriminant.sqrt() / a);
    }
}

pub fn ray_color(ray: &ray::Ray, world: &HittableList, depth: i32) -> Vec3A {
    if depth <= 0 {
        return Vec3A::ZERO;
    }

    if let Some(t) = world.hit(ray, 0.001, f32::INFINITY) {
        let target = t.position + t.normal + vec3::random_in_unit_sphere().normalize();
        return if let Some(r) = t.material.scatter(ray, &t) {
            r.attenuation * ray_color(&r.scattered, world, depth - 1)
        } else {
            Vec3A::ZERO
        };
    } else {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3A::ONE * (1.0 - t) + Vec3A::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    write_image("./output/glam.ppm".to_string()).unwrap();
}
