use crate::{ray::Ray, vec3::Point3};

use super::interval::Interval;

#[derive(Clone, Copy, Debug)]
pub struct Aabb {
    pub x_interval: Interval,
    pub y_interval: Interval,
    pub z_interval: Interval,
}

impl Aabb {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self {
            x_interval: x,
            y_interval: y,
            z_interval: z,
        }
    }

    pub const fn zero() -> Self {
        Self {
            x_interval: Interval::empty(),
            y_interval: Interval::empty(),
            z_interval: Interval::empty(),
        }
    }

    pub fn new_by_gap(point1: Point3, point2: Point3) -> Self {
        fn new_interval(value1: f32, value2: f32) -> Interval {
            if value1 <= value2 {
                Interval {
                    min: value1,
                    max: value2,
                }
            } else {
                Interval {
                    min: value2,
                    max: value1,
                }
            }
        }

        Self {
            x_interval: new_interval(point1.x, point2.x),
            y_interval: new_interval(point1.y, point2.y),
            z_interval: new_interval(point1.z, point2.z),
        }
    }
    pub fn merge_new(box_0: Self, box_1: Self) -> Self {
        Self {
            x_interval: Interval::merge(box_0.x_interval, box_1.x_interval),
            y_interval: Interval::merge(box_0.y_interval, box_1.y_interval),
            z_interval: Interval::merge(box_0.z_interval, box_1.z_interval),
        }
    }
}

impl Aabb {
    pub fn axis_index_interval(&self, index: usize) -> Interval {
        if index == 1 {
            self.y_interval
        } else if index == 2 {
            self.z_interval
        } else {
            self.x_interval
        }
    }

    pub fn hit(&self, ray: Ray, mut target_interval: Interval) -> bool {
        let ray_origin = ray.origin();
        let ray_direction = ray.direction();

        for axis_index in 0..=2 {
            let axis_interval = self.axis_index_interval(axis_index);
            let axis_divide = 1.0 / ray_direction[axis_index];

            let t0 = (axis_interval.min - ray_origin[axis_index]) * axis_divide;
            let t1 = (axis_interval.max - ray_origin[axis_index]) * axis_divide;

            if t0 < t1 {
                if t0 > target_interval.min {
                    target_interval.min = t0;
                }
                if t1 < target_interval.max {
                    target_interval.max = t1;
                }
            } else {
                if t1 > target_interval.min {
                    target_interval.min = t1;
                }
                if t0 < target_interval.max {
                    target_interval.max = t0;
                }
            }

            if target_interval.max <= target_interval.min {
                return false;
            }
        }
        true
    }

    pub fn merge(&self, other: Self) -> Self {
        Self {
            x_interval: Interval::merge(self.x_interval, other.x_interval),
            y_interval: Interval::merge(self.y_interval, other.y_interval),
            z_interval: Interval::merge(self.z_interval, other.z_interval),
        }
    }
}
