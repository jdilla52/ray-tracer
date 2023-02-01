mod camera;
mod error;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::hittable::{Hittable, HittableList};
use crate::ray::Ray;
use crate::vec3::Vec3;
use error::TracerResult;
use std::fs::File;
use std::io::Write;

pub fn write_image(path: String) -> TracerResult<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let camera = camera::Camera::default();
    let samples = 100;
    let max_depth = 50;

    let world = HittableList::new(vec![
        Box::new(sphere::Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(sphere::Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // camera
    let camera = camera::Camera::default();

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
                let ray = camera.as_ray(u, v);
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

pub fn ray_color(ray: &ray::Ray, world: &HittableList, depth: i32) -> vec3::Vec3 {
    if depth <= 0 {
        return vec3::Vec3::zero();
    }

    if let Some(t) = world.hit(ray, 0.0, f64::INFINITY) {
        let target = t.position + t.normal + Vec3::random_in_unit_sphere();
        ray_color(&Ray::new(t.position, target - t.position), world, depth - 1) * 0.5
    } else {
        let unit_direction = ray.direction.unit();
        let t = 0.5 * (unit_direction.y + 1.0);
        vec3::Vec3::one() * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn main() {
    write_image("./output/multiple_spheres.ppm".to_string()).unwrap();
}
