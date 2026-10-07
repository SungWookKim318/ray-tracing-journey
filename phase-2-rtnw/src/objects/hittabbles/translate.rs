use std::rc::Rc;

use crate::{
    objects::hittabbles::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::{aabb::Aabb, interval::Interval},
    vec3::Vec3,
};

struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bounding_box: Aabb,
}

impl Hittable for Translate {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());
        if !self.object.hit(offset_ray, ray_t, hit_record) {
            return false;
        }

        hit_record.point += self.offset;

        true
    }
    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bound_box = object.bounding_box();
        Self {
            object,
            offset,
            bounding_box: bound_box + offset,
        }
    }
}
