use crate::error::{TracerError, TracerResult};
use crate::geometry::aabb::Aabb;
use crate::geometry::bvh::{BvhNode, BvhNodeBuilder};
use crate::geometry::constant_medium::{ConstantMedium, ConstantMediumBuilder};
use crate::geometry::cornell_box::{CornellBox, CornellBoxBuilder};
use crate::geometry::hittable::{HittableList, HittableListBuilder};
use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::rotate_y::{RotateY, RotateYBuilder};
use crate::geometry::sphere::Sphere;
use crate::geometry::translate::{Translate, TranslateBuilder};
use crate::geometry::xy_rect::XyRect;
use crate::geometry::xz_rect::XzRect;
use crate::geometry::yz_rect::YzRect;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;

use std::rc::Rc;

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

// we wont support serialization of the bvh for now.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GeometryFile {
    Sphere(Sphere),
    XyRect(XyRect),
    XzRect(XzRect),
    YzRect(YzRect),
    Translate(TranslateBuilder),
    RotateY(RotateYBuilder),
    CornellBox(CornellBoxBuilder),
    BvhNode(BvhNodeBuilder),
    ConstantMedium(ConstantMediumBuilder),
    MovingSphere(MovingSphere),
    HittableList(HittableListBuilder),
}

impl TryInto<Geometry> for GeometryFile {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Geometry> {
        match self {
            GeometryFile::Sphere(sphere) => Ok(Geometry::Sphere(sphere)),
            GeometryFile::XyRect(xy_rect) => Ok(Geometry::XyRect(xy_rect)),
            GeometryFile::XzRect(xz_rect) => Ok(Geometry::XzRect(xz_rect)),
            GeometryFile::YzRect(yz_rect) => Ok(Geometry::YzRect(yz_rect)),
            GeometryFile::Translate(translate) => Ok(translate.try_into()?),
            GeometryFile::RotateY(rotate_y) => Ok(rotate_y.try_into()?),
            GeometryFile::CornellBox(cornell_box) => Ok(cornell_box.try_into()?),
            GeometryFile::ConstantMedium(constant_medium) => Ok(constant_medium.try_into()?),
            GeometryFile::MovingSphere(moving_sphere) => Ok(Geometry::MovingSphere(moving_sphere)),
            GeometryFile::HittableList(hittable_list) => Ok(hittable_list.try_into()?),
            GeometryFile::BvhNode(bvh_node) => Ok(bvh_node.try_into()?),
        }
    }
}

impl TryInto<Box<Geometry>> for Box<GeometryFile> {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Box<Geometry>> {
        match *self {
            GeometryFile::Sphere(sphere) => Ok(Box::new(Geometry::Sphere(sphere))),
            GeometryFile::XyRect(xy_rect) => Ok(Box::new(Geometry::XyRect(xy_rect))),
            GeometryFile::XzRect(xz_rect) => Ok(Box::new(Geometry::XzRect(xz_rect))),
            GeometryFile::YzRect(yz_rect) => Ok(Box::new(Geometry::YzRect(yz_rect))),
            GeometryFile::Translate(translate) => Ok(Box::new(translate.try_into()?)),
            GeometryFile::RotateY(rotate_y) => Ok(Box::new(rotate_y.try_into()?)),
            GeometryFile::CornellBox(cornell_box) => Ok(Box::new(cornell_box.try_into()?)),
            GeometryFile::ConstantMedium(constant_medium) => {
                Ok(Box::new(constant_medium.try_into()?))
            }
            GeometryFile::MovingSphere(moving_sphere) => {
                Ok(Box::new(Geometry::MovingSphere(moving_sphere)))
            }
            GeometryFile::HittableList(hittable_list) => Ok(Box::new(hittable_list.try_into()?)),
            GeometryFile::BvhNode(bvh_node) => Ok(Box::new(bvh_node.try_into()?)),
        }
    }
}

impl TryInto<Rc<Geometry>> for Box<GeometryFile> {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Rc<Geometry>> {
        match *self {
            GeometryFile::Sphere(sphere) => Ok(Rc::new(Geometry::Sphere(sphere))),
            GeometryFile::XyRect(xy_rect) => Ok(Rc::new(Geometry::XyRect(xy_rect))),
            GeometryFile::XzRect(xz_rect) => Ok(Rc::new(Geometry::XzRect(xz_rect))),
            GeometryFile::YzRect(yz_rect) => Ok(Rc::new(Geometry::YzRect(yz_rect))),
            GeometryFile::Translate(translate) => Ok(Rc::new(translate.try_into()?)),
            GeometryFile::RotateY(rotate_y) => Ok(Rc::new(rotate_y.try_into()?)),
            GeometryFile::CornellBox(cornell_box) => Ok(Rc::new(cornell_box.try_into()?)),
            GeometryFile::ConstantMedium(constant_medium) => {
                Ok(Rc::new(constant_medium.try_into()?))
            }
            GeometryFile::MovingSphere(moving_sphere) => {
                Ok(Rc::new(Geometry::MovingSphere(moving_sphere)))
            }
            GeometryFile::HittableList(hittable_list) => Ok(Rc::new(hittable_list.try_into()?)),
            GeometryFile::BvhNode(bvh_node) => Ok(Rc::new(bvh_node.try_into()?)),
        }
    }
}

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
