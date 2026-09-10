use std::rc::Rc;

use crate::{
    objects::{
        hittable::{HitRecord, Hittable},
        materials::material::Material,
    },
    ray::Ray,
    utils::{aabb::Aabb, interval::Interval},
    vec3::{Point3, Vec3},
};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Rc<dyn Material>,
    bounding_box: Aabb,
}

impl Sphere {
    pub fn new(static_center: Point3, radius: f32, material: Rc<dyn Material>) -> Self {
        if radius <= 0.0 {
            panic!("radius should be over 0.");
        }

        let radius_vector = Vec3::with_scalar(radius);
        Self {
            center: Ray::new(static_center, Vec3::zero(), 0.0),
            radius,
            material,
            bounding_box: Aabb::new_by_gap(
                static_center - radius_vector,
                static_center + radius_vector,
            ),
        }
    }

    pub fn moved_new(
        center1: Point3,
        center2: Point3,
        radius: f32,
        material: Rc<dyn Material>,
    ) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.0);
        let radius_vector = Vec3::with_scalar(radius);
        let box_at_zero = Aabb::new_by_gap(
            center.at(0.0) - radius_vector,
            center.at(0.0) + radius_vector,
        );
        let box_at_final = Aabb::new_by_gap(
            center.at(1.0) - radius_vector,
            center.at(1.0) + radius_vector,
        );
        Self {
            center,
            radius,
            material,
            bounding_box: Aabb::merge_new(box_at_zero, box_at_final),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval, record: &mut HitRecord) -> bool {
        let current_center = self.center.at(ray.time());
        let origin_center = current_center - ray.origin();
        let a = ray.direction().length_squared();
        if a == 0.0 {
            return false;
        }

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
        let outward_normal = record.normal;
        record.set_face_normal(&ray, &outward_normal);
        record.material = Rc::clone(&self.material);
        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}
