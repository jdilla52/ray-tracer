use crate::geometry::aabb::Aabb;
use crate::geometry::hittable::{HitRecord, Hittable, HittableList};
use crate::geometry::xy_rect::XyRect;
use crate::geometry::xz_rect::XzRect;
use crate::geometry::yz_rect::YzRect;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;
use std::rc::Rc;

pub struct CornellBox {
    pub min: Vec3A,
    pub max: Vec3A,
    pub sides: HittableList,
}

impl CornellBox {
    pub fn new(p0: Vec3A, p1: Vec3A, material: Rc<dyn Material>) -> Self {
        let sides = HittableList::new(vec![
            Rc::new(XyRect::new(p0.x, p1.x, p0.y, p1.y, p1.z, material.clone())),
            Rc::new(XyRect::new(p0.x, p1.x, p0.y, p1.y, p0.z, material.clone())),
            Rc::new(XzRect::new(p0.x, p1.x, p0.z, p1.z, p1.y, material.clone())),
            Rc::new(XzRect::new(p0.x, p1.x, p0.z, p1.z, p0.y, material.clone())),
            Rc::new(YzRect::new(p0.y, p1.y, p0.z, p1.z, p1.x, material.clone())),
            Rc::new(YzRect::new(p0.y, p1.y, p0.z, p1.z, p0.x, material.clone())),
        ]);

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for CornellBox {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(Aabb::new(self.min, self.max))
    }
}
