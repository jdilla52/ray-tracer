use crate::ray::Ray;
use crate::vec3;
use glam::Vec3A;

pub struct Camera {
    pub origin: Vec3A,
    pub lower_left_corner: Vec3A,
    pub horizontal: Vec3A,
    pub vertical: Vec3A,
    pub u: Vec3A,
    pub v: Vec3A,
    pub w: Vec3A,
    pub lens_radius: f32,
}

impl Camera {
    pub(crate) fn as_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }

    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        vup: Vec3A,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let view_height = 2.0 * h;
        let view_width = aspect_ratio * view_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u * focus_dist * view_width;
        let vertical = v * focus_dist * view_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}

// impl Default for Camera {
//     fn default() -> Self {
//         let aspect_ratio = 16.0 / 9.0;
//         let viewport_height = 2.0;
//         let viewport_width = aspect_ratio * viewport_height;
//         let focal_length = 1.0;
//
//         let origin = Vec3A::ZERO;
//         let horizontal = Vec3A::new(viewport_width, 0.0, 0.0);
//         let vertical = Vec3A::new(0.0, viewport_height, 0.0);
//         Self {
//             origin,
//             lower_left_corner: origin
//                 - horizontal / 2.0
//                 - vertical / 2.0
//                 - Vec3A::new(0., 0., focal_length),
//             horizontal,
//             vertical,
//         }
//     }
// }

// impl Into<Ray> for Camera {
//     fn into(self) -> Ray {
//         Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin)
//     }
// }
