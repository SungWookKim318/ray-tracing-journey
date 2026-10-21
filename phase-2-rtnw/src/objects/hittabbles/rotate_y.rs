use std::rc::Rc;

use itertools::iproduct;

use crate::{
    objects::hittabbles::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::{aabb::Aabb, interval::Interval, math_constant::INFINITY},
    vec3::{Point3, Vec3},
};

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f32,
    cos_theta: f32,
    bounding_box: Aabb,
}

impl Hittable for RotateY {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        // Transform the ray from world space to object space.
        let origin = Point3::new(
            self.cos_theta * ray.origin().x - self.sin_theta * ray.origin().z,
            ray.origin().y,
            self.sin_theta * ray.origin().x + self.cos_theta * ray.origin().z,
        );

        let direction = Vec3::new(
            self.cos_theta * ray.direction().x - self.sin_theta * ray.direction().z,
            ray.direction().y,
            self.sin_theta * ray.direction().x + self.cos_theta * ray.direction().z,
        );

        let rotate_ray = Ray::new(origin, direction, ray.time());

        if !self.object.hit(rotate_ray, ray_t, hit_record) {
            return false;
        }

        hit_record.point = Point3::new(
            self.cos_theta * hit_record.point.x + self.sin_theta * hit_record.point.z,
            hit_record.point.y,
            -self.sin_theta * hit_record.point.x + self.cos_theta * hit_record.point.z,
        );

        hit_record.normal = Vec3::new(
            self.cos_theta * hit_record.normal.x + self.sin_theta * hit_record.normal.z,
            hit_record.normal.y,
            -self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z,
        );

        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, degree: f32) -> Self {
        let radius = degree.to_radians();

        let sin_theta = radius.sin();
        let cos_theta = radius.cos();
        let mut bounding_box = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        for (i, j, k) in iproduct!(0..1, 0..1, 0..1).map(|(i, j, k)| (i as f32, j as f32, k as f32))
        {
            let x = i * bounding_box.x_interval.max + (1.0 - i) * bounding_box.x_interval.min;
            let y = j * bounding_box.y_interval.max + (1.0 - j) * bounding_box.y_interval.min;
            let z = k * bounding_box.z_interval.max + (1.0 - k) * bounding_box.z_interval.min;

            let new_x = cos_theta * x + sin_theta * z;
            let new_z = -sin_theta * x + cos_theta * z;

            let tester = Vec3::new(new_x, y, new_z);
            for index in 0..2 {
                min[index] = min[index].min(tester[index]);
                max[index] = max[index].max(tester[index]);
            }
        }

        bounding_box = Aabb::new_by_gap(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bounding_box,
        }
    }
}
