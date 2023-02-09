use crate::geometry::aabb::Aabb;
use crate::geometry::Hittable;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::isotropic::Isotropic;
use crate::material::Material;
use glam::Vec3A;
use log::debug;


#[derive(Clone)]
pub struct ConstantMedium {
    pub boundary: Box<dyn Hittable>,
    pub phase_function: usize,
    pub neg_inv_density: f32,
}

impl ConstantMedium {
    pub fn new(b: Box<dyn Hittable>, d: f32, phase_function: usize) -> Self {
        Self {
            boundary: b,
            neg_inv_density: -1.0 / d,
            phase_function,
        }
    }

    // pub fn new_from_density(b: Box<dyn Hittable>, d: f32, c: Vec3A) -> Self {
    //     Self {
    //         boundary: b,
    //         neg_inv_density: -1.0 / d,
    //         phase_function: Box::new(Isotropic::new_color(c)),
    //     }
    // }
}

// only works for convex shapes
impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        let debug_threshold = 0.00001;

        // Intersect ray with bounding box.
        if let Some(rec1) = self.boundary.hit(r, -f32::INFINITY, f32::INFINITY) {
            // Bounce next ray off bounding box.
            if let Some(rec2) = self.boundary.hit(r, rec1.root + 0.0001, f32::INFINITY) {
                let t1 = rec1.root.max(t_min);
                let t2 = rec2.root.min(t_max);

                if t1 >= t2 {
                    return None;
                }
                let t1 = t1.max(0.0);

                // Ray length inside bounding box.
                let ray_length = r.direction.length();
                let distance_inside_boundary = (t2 - t1) * ray_length;
                let hit_distance = self.neg_inv_density * rand::random::<f32>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = t1 + hit_distance / ray_length;
                if rand::random::<f32>() < debug_threshold {
                    debug!("hit_distance = {}, rec.t = {}", hit_distance, t);
                }
                Some(HitRecord {
                    root: t,
                    position: r.at(t),
                    normal: Default::default(),
                    front_face: true,
                    material_index: self.phase_function,
                    u: 0.0,
                    v: 0.0,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        self.boundary.bounding_box(time0, time1)
    }
}
