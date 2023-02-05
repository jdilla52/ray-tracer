use crate::error::TracerError::BvhBoundingBoxError;
use crate::error::TracerResult;
use crate::geometry::aabb::Aabb;
use crate::geometry::Hittable;
use crate::intersection::hit_record::HitRecord;
use crate::intersection::ray::Ray;
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub bounding_box: Aabb,
}

impl BvhNode {
    pub fn new(left: Rc<dyn Hittable>, right: Rc<dyn Hittable>, bounding_box: Aabb) -> BvhNode {
        BvhNode {
            left,
            right,
            bounding_box,
        }
    }

    pub fn from_list(list: Vec<Rc<dyn Hittable>>, t0: f32, t1: f32) -> TracerResult<BvhNode> {
        let mut list = list;
        let axis = Axis::random();
        let object_span = list.len();
        let (left, right) = if object_span == 1 {
            //if there's just one object put it in both left and right
            (list[0].clone(), list[0].clone())
        } else if object_span == 2 {
            if compare_boxes(&list[0], &list[1], axis) {
                (list[0].clone(), list[1].clone())
            } else {
                (list[1].clone(), list[0].clone())
            }
        } else {
            list.sort_by(|a, b| sort_boxes(a, b, axis));
            let mid = object_span / 2;
            let mut left_list = list[0..mid].to_vec();
            let mut right_list = list[mid..].to_vec();
            let left: Rc<dyn Hittable> = Rc::new(BvhNode::from_list(left_list, t0, t1)?);
            let right: Rc<dyn Hittable> = Rc::new(BvhNode::from_list(right_list, t0, t1)?);
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

fn sort_boxes(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: Axis) -> Ordering {
    if compare_boxes(a, b, axis) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn compare_boxes(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis: Axis) -> bool {
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
    fn bounding_box(&self, t0: f32, t1: f32) -> Option<Aabb> {
        Some(self.bounding_box.clone())
    }
}
