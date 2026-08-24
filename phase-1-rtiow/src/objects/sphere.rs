use crate::{
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    vec3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f32,
}

impl Sphere {
    pub const fn new(center: Point3, radius: f32) -> Self {
        Self { center, radius }
    }

    pub const fn zero() -> Self {
        Self {
            center: Point3::zero(),
            radius: 0.0,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, record: &mut HitRecord) -> bool {
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
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrt_discriminant) / a;
            if root <= ray_tmin || ray_tmax <= root {
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
