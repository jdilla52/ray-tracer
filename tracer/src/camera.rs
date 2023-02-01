use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    pub(crate) fn as_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        )
    }

    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let view_height = 2.0 * h;
        let view_width = aspect_ratio * view_height;

        let w = (look_from - look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

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

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
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
//         let origin = Vec3::zero();
//         let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
//         let vertical = Vec3::new(0.0, viewport_height, 0.0);
//         Self {
//             origin,
//             lower_left_corner: origin
//                 - horizontal / 2.0
//                 - vertical / 2.0
//                 - Vec3::new(0., 0., focal_length),
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
