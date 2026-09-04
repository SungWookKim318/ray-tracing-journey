use crate::ray::Ray;
use crate::objects::hittable::{HitRecord, Hittable};
use crate::utils::interval::Interval;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub const fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut is_hit_anything = false;
        let mut closet_tmax = ray_t.max;

        for object in &self.objects {
            // 바로 넘겨주면 안에서 알아서 값이 바뀔테니, 그걸 그냥 계속 쓰는 방향으로 하기
            if object.hit(ray, Interval::new(ray_t.min, closet_tmax), hit_record) {
                is_hit_anything = true;
                closet_tmax = hit_record.t;
            }
        }
        is_hit_anything
    }
}
