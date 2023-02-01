mod error;
mod vec3;
mod ray;

use std::fs::File;
use std::io::Write;
use error::TracerResult;

pub fn write_image(width: i32, height: i32, path: String) -> TracerResult<()>{

    let mut output = File::create(path)?;
    writeln!(&mut output, "P3\n{} {}\n255", width, height)?;
    for j in (0..height-1).rev(){
        println!("Scanlines remaining: {}", height - j - 1);
        for i in  0..width{
            let color_vec = vec3::Vec3::new(i as f64 / (width-1) as f64, j as f64 / (height-1) as f64, 0.25);
            writeln!(&mut output, "{}", color_vec.as_color())?;
        }
    }

    Ok(())
}

pub fn ray_color(ray: &ray::Ray) -> vec3::Vec3 {
    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    vec3::Vec3::one().scale(1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0).scale(t)
}

fn main() {
   write_image(200, 200, "./image.ppm".to_string()).unwrap();
}
