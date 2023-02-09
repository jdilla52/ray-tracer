use crate::geometry::aabb::Aabb;
use crate::geometry::Hittable;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use crate::material::Material;
use glam::Vec3A;



#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3A,
    pub radius: f32,
    pub material_index: usize,
}


pub fn get_sphere_uv(p: Vec3A) -> (f32, f32) {
    let theta = -p.y.acos();
    let phi = -p.z.atan2(p.x) + std::f32::consts::PI;
    (phi / (TWO_PI), theta / std::f32::consts::PI)
}

static TWO_PI: f32 = std::f32::consts::PI * 2.0;

impl Sphere {
    pub fn new(center: Vec3A, radius: f32, material_index: usize) -> Sphere {
        Sphere {
            center,
            radius,
            material_index,
        }
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Vec3A::splat(self.radius),
            self.center + Vec3A::splat(self.radius),
        ))
    }

    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let position = ray.at(root);
        let outward_normal = (position - self.center) / self.radius;
        if ray.direction.dot(outward_normal) < 0.0 {
            let (u, v) = get_sphere_uv(outward_normal);
            Some(HitRecord {
                root,
                position,
                normal: outward_normal,
                front_face: true,
                material_index: self.material_index,
                u,
                v,
            })
        } else {
            let outward_normal = -outward_normal;
            let (u, v) = get_sphere_uv(outward_normal);
            Some(HitRecord {
                root,
                position,
                normal: -outward_normal,
                front_face: false,
                material_index: self.material_index,
                u,
                v,
            })
        }
    }
}
