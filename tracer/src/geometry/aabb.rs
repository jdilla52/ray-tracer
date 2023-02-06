use crate::intersection::ray::Ray;
use glam::Vec3A;


#[derive(Clone, Copy)]
pub struct Aabb {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl Aabb {
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self { min, max }
    }

    pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Self {
        let small = Vec3A::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Vec3A::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        Self::new(small, big)
    }

    pub fn slow_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let t0 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .min((self.max[a] - ray.origin[a]) / ray.direction[a]);
            let t1 = ((self.min[a] - ray.origin[a]) / ray.direction[a])
                .max((self.max[a] - ray.origin[a]) / ray.direction[a]);
            if t0.min(t_min) <= t1.max(t_max) {
                return false;
            }
        }
        true
    }
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let t1 = (self.max[a] - ray.origin[a]) * inv_d;

            // isn't it's faster to branch here than memswap
            // if (invD < 0.0f)
            // std::swap(t0, t1);
            if inv_d < 0.0 {
                let tmin = if t1 > t_min { t1 } else { t_min };
                let tmax = if t0 < t_max { t0 } else { t_max };
                if tmax <= tmin {
                    return false;
                }
            } else {
                let tmin = if t0 > t_min { t0 } else { t_min };
                let tmax = if t1 < t_max { t1 } else { t_max };
                if tmax <= tmin {
                    return false;
                }
            }
        }
        true
    }
}
