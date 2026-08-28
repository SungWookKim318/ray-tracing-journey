use std::rc::Rc;

use crate::{
    objects::{
        hittable::{HitRecord, Hittable},
        materials::material::{Material, NoneMaterial},
    },
    ray::Ray,
    utils::interval::Interval,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn zero() -> Self {
        Self {
            center: Point3::zero(),
            radius: 0.0,
            material: Rc::new(NoneMaterial::new()),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let origin_center = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(origin_center);
        let c = origin_center.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();

        let mut root = (h - sqrt_discriminant) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        record.normal = (record.point - self.center) / self.radius;
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(&ray, &outward_normal);
        true
    }
}
