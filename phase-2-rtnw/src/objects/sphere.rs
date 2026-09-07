use std::rc::Rc;

use crate::{
    objects::{
        hittable::{HitRecord, Hittable},
        materials::material::{Material, NoneMaterial},
    },
    ray::Ray,
    utils::interval::Interval,
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f32, material: Rc<dyn Material>) -> Self {
        Self {
            center: Ray::new(static_center, Vec3::zero(), 0.0),
            radius,
            material,
        }
    }

    pub fn moved_new(
        center1: Point3,
        center2: Point3,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1, 0.0),
            radius,
            material,
        }
    }

    pub fn zero() -> Self {
        Self {
            center: Ray::zero(),
            radius: 0.0,
            material: Rc::new(NoneMaterial::new()),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let origin_center = current_center - ray.origin();
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
        record.normal = (record.point - current_center) / self.radius;
        let outward_normal = record.normal.clone();
        record.set_face_normal(&ray, &outward_normal);
        record.material = Rc::clone(&self.material);
        true
    }
}
