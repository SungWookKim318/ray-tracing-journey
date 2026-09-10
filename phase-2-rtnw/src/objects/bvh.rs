use std::cmp::Ordering;
use std::rc::Rc;

use super::hittable::Hittable;
use crate::objects::hittable_list::HittableList;
use crate::ray::Ray;
use crate::utils::{aabb::Aabb, interval::Interval};

pub struct BvhNode {
    left: Option<Rc<dyn Hittable>>,
    right: Option<Rc<dyn Hittable>>,
    bounding_box: Aabb,
}

impl BvhNode {
    pub fn new_root(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        if len == 0 {
            return Self::new_empty_node();
        }
        Self::generate_leafs(&mut list.objects, 0, len)
    }

    fn new_empty_node() -> Self {
        Self {
            left: Option::None,
            right: Option::None,
            bounding_box: Aabb::empty(),
        }
    }

    fn generate_leafs(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let object_span = end - start;
        if object_span == 0 {
            eprintln!(
                "object_span is zero will be empty tree, start: {}, end: {}",
                start, end
            );
            return BvhNode::new_empty_node();
        }

        // let mut rng = rand::rng();
        // let axis_index = rng.random_range(0..=2usize);
        let mut new_bounding_box = Aabb::empty();
        for object in objects[start..end].iter() {
            new_bounding_box.extend(object.bounding_box());
        }
        let axis_index = new_bounding_box.longest_axis();

        let comparator = if axis_index == 0 {
            BvhNode::box_x_compare
        } else if axis_index == 1 {
            BvhNode::box_y_compare
        } else {
            BvhNode::box_z_compare
        };

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        if object_span == 1 {
            left = objects[start].clone();
            right = objects[start].clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span / 2;
            left = Rc::new(BvhNode::generate_leafs(objects, start, mid));
            right = Rc::new(BvhNode::generate_leafs(objects, mid, end));
        }

        Self {
            left: Option::Some(left.clone()),
            right: Option::Some(right.clone()),
            bounding_box: new_bounding_box,
        }
    }

    fn box_compare(lhs: &Rc<dyn Hittable>, rhs: &Rc<dyn Hittable>, axis_index: usize) -> Ordering {
        let left_axis_interval = lhs.bounding_box().axis_index_interval(axis_index);
        let right_axis_interval = rhs.bounding_box().axis_index_interval(axis_index);

        if left_axis_interval.min < right_axis_interval.min {
            Ordering::Less
        } else if left_axis_interval.min == right_axis_interval.min {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }

    fn box_x_compare(lhs: &Rc<dyn Hittable>, rhs: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(lhs, rhs, 0)
    }
    fn box_y_compare(lhs: &Rc<dyn Hittable>, rhs: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(lhs, rhs, 1)
    }
    fn box_z_compare(lhs: &Rc<dyn Hittable>, rhs: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(lhs, rhs, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut super::hittable::HitRecord) -> bool {
        if !self.bounding_box.hit(ray, ray_t) {
            return false;
        }

        let is_hit_left = self
            .left
            .as_ref()
            .is_some_and(|left| left.hit(ray, ray_t, hit_record));

        let hit_right_max = if is_hit_left { hit_record.t } else { ray_t.max };
        let hit_right_interval = Interval::new(ray_t.min, hit_right_max);
        // let is_hit_right = self.right.hit(ray, hit_right_interval, hit_record);
        let is_hit_right = self
            .right
            .as_ref()
            .is_some_and(|right| right.hit(ray, hit_right_interval, hit_record));

        is_hit_left || is_hit_right
    }
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
