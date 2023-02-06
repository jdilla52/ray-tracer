use crate::vec3;
use glam::Vec3A;
use rand::Rng;

pub fn random_range(min: f32, max: f32) -> Vec3A {
    let mut rng = rand::thread_rng(); // TODO: move once threading
    rng.gen::<Vec3A>() * (max - min) + min
}

pub fn random_in_unit_sphere() -> Vec3A {
    let mut rng = rand::thread_rng(); // TODO: move once threading
    loop {
        let p = rng.gen::<Vec3A>() * 2.0 - 1.0;
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_hemisphere(value: Vec3A) -> Vec3A {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(value) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
pub fn random_in_unit_disk() -> Vec3A {
    let mut rng = rand::thread_rng(); // TODO: move once threading
    loop {
        let p = Vec3A::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn reflect(value: Vec3A, normal: Vec3A) -> Vec3A {
    value - normal * 2.0 * value.dot(normal)
}

pub fn refract(value: Vec3A, normal: Vec3A, etai_over_etat: f32) -> Vec3A {
    let cos_theta = (-value).dot(normal).min(1.0);
    let r_out_perp = (value + normal * cos_theta) * etai_over_etat;
    let r_out_parallel = normal * -(r_out_perp.length_squared() - 1.0).abs().sqrt();
    r_out_perp + r_out_parallel
}

pub fn as_aggregated_color(value: Vec3A, samples: i32) -> String {
    let scaled = vec3::sqrt(value * (1.0 / samples as f32));
    let clamped = scaled.clamp(Vec3A::ZERO, Vec3A::new(0.999, 0.999, 0.999)) * 256.0;
    format!("{} {} {}", clamped.x, clamped.y, clamped.z)
}

pub fn sqrt(value: Vec3A) -> Vec3A {
    Vec3A::new(value.x.sqrt(), value.y.sqrt(), value.z.sqrt())
}

pub fn unit(value: Vec3A) -> Vec3A {
    value / value.length()
}
