mod error;
mod vec3;
mod ray;

use std::fs::File;
use std::io::Write;
use error::TracerResult;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn write_image(path: String) -> TracerResult<()>{
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = vec3::Vec3::zero();
    let horizontal = vec3::Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = vec3::Vec3::new(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal.scale(0.5) - vertical.scale(0.5) - vec3::Vec3::new(0.0, 0.0, focal_length);

    let mut output = File::create(path)?;
    writeln!(&mut output, "P3\n{} {}\n255", image_width, image_height)?;
    for j in (0..image_height-1).rev(){
        println!("Scanlines remaining: {}", image_height - j - 1);
        for i in  0..image_width{
            let u = i as f64 / (image_width-1) as f64;
            let v = j as f64 / (image_height-1) as f64;

            let ray = ray::Ray::new(origin, lower_left_corner + horizontal * u + vertical * v - origin);
            let color = ray_color(&ray);
            writeln!(&mut output, "{}", color.as_color())?;
        }
    }

    Ok(())
}



pub fn ray_color(ray: &ray::Ray) -> vec3::Vec3 {
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    vec3::Vec3::one() * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t
}

fn main() {
   write_image("./output/red_sphere.ppm".to_string()).unwrap();
}
