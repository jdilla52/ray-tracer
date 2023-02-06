use crate::error::TracerError::BvhBoundingBoxError;
use crate::error::{TracerError, TracerResult};
use crate::geometry::aabb::Aabb;
use crate::geometry::{Geometry, GeometryFile, Hittable};
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use std::cmp::Ordering;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BvhNodeBuilder {
    pub left: Box<GeometryFile>,
    pub right: Box<GeometryFile>,
    pub bounding_box: Aabb,
}

impl TryInto<Geometry> for BvhNodeBuilder {
    type Error = TracerError;

    fn try_into(self) -> TracerResult<Geometry> {
        let l = self.left.try_into()?;
        let r = self.right.try_into()?;
        Ok(Geometry::BvhNode(BvhNode::new(l, r, self.bounding_box)))
    }
}

pub struct BvhNode {
    pub left: Rc<Geometry>,
    pub right: Rc<Geometry>,
    pub bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(left: Rc<Geometry>, right: Rc<Geometry>, bounding_box: Aabb) -> BvhNode {
        BvhNode {
            left,
            right,
            bounding_box,
        }
    }

    // note that the rest of the geometry objects are stored in a flat list in the render context
    pub fn from_list(list: Vec<Rc<Geometry>>, t0: f32, t1: f32) -> TracerResult<BvhNode> {
        let mut list = list;
        let axis = Axis::random();
        let object_span = list.len();
        let (left, right) = if object_span == 1 {
            //if there's just one object put it in both left and right
            (list[0].to_owned(), list[0].to_owned())
        } else if object_span == 2 {
            if compare_boxes(&list[0], &list[1], axis) {
                (list[0].to_owned(), list[1].to_owned())
            } else {
                (list[1].to_owned(), list[0].to_owned())
            }
        } else {
            list.sort_by(|a, b| sort_boxes(a, b, axis));
            let mid = object_span / 2;
            let left_list = list[0..mid].to_vec();
            let right_list = list[mid..].to_vec();
            let left: Rc<Geometry> =
                Rc::new(Geometry::BvhNode(BvhNode::from_list(left_list, t0, t1)?));
            let right: Rc<Geometry> =
                Rc::new(Geometry::BvhNode(BvhNode::from_list(right_list, t0, t1)?));
            (left, right)
        };

        let box_left = left.bounding_box(t0, t1).ok_or(BvhBoundingBoxError)?;
        let box_right = right.bounding_box(t0, t1).ok_or(BvhBoundingBoxError)?;
        let bounding_box = Aabb::surrounding_box(&box_left, &box_right);
        Ok(BvhNode::new(left, right, bounding_box))
    }
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn next(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
    fn random() -> Axis {
        match (3.0 * rand::random::<f32>()) as usize {
            0 => Axis::X,
            1 => Axis::Y,
            2 => Axis::Z,
            _ => Axis::X,
        }
    }
}

fn sort_boxes(a: &Rc<Geometry>, b: &Rc<Geometry>, axis: Axis) -> Ordering {
    if compare_boxes(a, b, axis) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn compare_boxes(a: &Rc<Geometry>, b: &Rc<Geometry>, axis: Axis) -> bool {
    let box_a = a.bounding_box(0.0, 0.0).unwrap();
    let box_b = b.bounding_box(0.0, 0.0).unwrap();
    match axis {
        Axis::X => box_a.min.x < box_b.min.x,
        Axis::Y => box_a.min.y < box_b.min.y,
        Axis::Z => box_a.min.z < box_b.min.z,
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.bounding_box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(r, t_min, t_max);
        let hit_right = self.right.hit(r, t_min, t_max);

        match (hit_left, hit_right) {
            (Some(left), Some(right)) => {
                if left.root < right.root {
                    Some(left)
                } else {
                    Some(right)
                }
            }
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (None, None) => None,
        }
    }
    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<Aabb> {
        Some(self.bounding_box.clone())
    }
}
