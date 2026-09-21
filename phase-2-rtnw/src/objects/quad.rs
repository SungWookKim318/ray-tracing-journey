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

struct Quad {
    origin: Point3,
    u: Vec3,
    v: Vec3,
    material: Rc<dyn Material>,
    bounding_box: Aabb,
    normal: Vec3,
    d: f32,
}

impl Hittable for Quad {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let denom: f32 = self.normal.dot(ray.direction());
        if denom.abs() < 0.0001 {
            return false;
        }

        let t = (self.d - self.normal.dot(ray.origin())) / denom;
        if !ray_t.contain(t) {
            return false;
        }

        let intersection = ray.at(t);

        hit_record.t = t;
        hit_record.point = intersection;
        hit_record.material = self.material.clone();
        hit_record.set_face_normal(&ray, &self.normal);

        false
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

impl Quad {
    fn new(origin: Point3, u: Vec3, v: Vec3, material: Rc<dyn Material>) -> Self {
        let bounding_box = Self::get_new_bounding_box(origin, u, v);
        let normal = u.cross(v).normalize();
        let d = normal.dot(origin);

        Self {
            origin,
            u,
            v,
            material,
            bounding_box,
            normal,
            d,
        }
    }
}

impl Quad {
    fn update_bounding_box(&mut self) {
        let diagonal_box_1 = Aabb::new_by_gap(self.origin, self.origin + self.u + self.v);
        let diagonal_box_2 = Aabb::new_by_gap(self.origin + self.u, self.origin + self.v);

        self.bounding_box = Aabb::merge_new(diagonal_box_1, diagonal_box_2);
    }

    fn get_new_bounding_box(origin: Point3, u: Vec3, v: Vec3) -> Aabb {
        let diagonal_box_1 = Aabb::new_by_gap(origin, origin + u + v);
        let diagonal_box_2 = Aabb::new_by_gap(origin + u, origin + v);

        Aabb::merge_new(diagonal_box_1, diagonal_box_2)
    }
}
