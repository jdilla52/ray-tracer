use crate::geometry::aabb::Aabb;
use crate::geometry::bvh::BvhNode;
use crate::geometry::constant_medium::ConstantMedium;
use crate::geometry::cornell_box::CornellBox;
use crate::geometry::hittable::HittableList;
use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::rotate_y::RotateY;
use crate::geometry::sphere::Sphere;
use crate::geometry::translate::Translate;
use crate::geometry::xy_rect::XyRect;
use crate::geometry::xz_rect::XzRect;
use crate::geometry::yz_rect::YzRect;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;

pub mod aabb;
pub mod bvh;
pub mod constant_medium;
pub mod cornell_box;
pub mod hittable;
pub mod moving_sphere;
pub mod rotate_y;
pub mod sphere;
pub mod translate;
pub mod xy_rect;
pub mod xz_rect;
pub mod yz_rect;


pub enum Geometry {
    Sphere(Sphere),
    XyRect(XyRect),
    XzRect(XzRect),
    YzRect(YzRect),
    Translate(Translate),
    RotateY(RotateY),
    CornellBox(CornellBox),
    ConstantMedium(ConstantMedium),
    BvhNode(BvhNode),
    MovingSphere(MovingSphere),
    HittableList(HittableList),
}

impl Hittable for Geometry {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Geometry::XyRect(xy_rect) => xy_rect.hit(ray, t_min, t_max),
            Geometry::XzRect(xz_rect) => xz_rect.hit(ray, t_min, t_max),
            Geometry::YzRect(yz_rect) => yz_rect.hit(ray, t_min, t_max),
            Geometry::Translate(translate) => translate.hit(ray, t_min, t_max),
            Geometry::RotateY(rotate_y) => rotate_y.hit(ray, t_min, t_max),
            Geometry::CornellBox(cornell_box) => cornell_box.hit(ray, t_min, t_max),
            Geometry::ConstantMedium(constant_medium) => constant_medium.hit(ray, t_min, t_max),
            Geometry::BvhNode(bvh_node) => bvh_node.hit(ray, t_min, t_max),
            Geometry::MovingSphere(moving_sphere) => moving_sphere.hit(ray, t_min, t_max),
            Geometry::HittableList(hittable_list) => hittable_list.hit(ray, t_min, t_max),
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        match self {
            Geometry::Sphere(sphere) => sphere.bounding_box(t0, t1),
            Geometry::XyRect(xy_rect) => xy_rect.bounding_box(t0, t1),
            Geometry::XzRect(xz_rect) => xz_rect.bounding_box(t0, t1),
            Geometry::YzRect(yz_rect) => yz_rect.bounding_box(t0, t1),
            Geometry::Translate(translate) => translate.bounding_box(t0, t1),
            Geometry::RotateY(rotate_y) => rotate_y.bounding_box(t0, t1),
            Geometry::CornellBox(cornell_box) => cornell_box.bounding_box(t0, t1),
            Geometry::ConstantMedium(constant_medium) => constant_medium.bounding_box(t0, t1),
            Geometry::BvhNode(bvh_node) => bvh_node.bounding_box(t0, t1),
            Geometry::MovingSphere(moving_sphere) => moving_sphere.bounding_box(t0, t1),
            Geometry::HittableList(hittable_list) => hittable_list.bounding_box(t0, t1),
        }
    }
}

// enum Hittable
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb>;
}

