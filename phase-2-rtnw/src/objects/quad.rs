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

pub struct Quad {
    origin: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
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

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = ray.at(t);
        let planar_hit_point_vector = intersection - self.origin;
        let alpha = self.w.dot(planar_hit_point_vector.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hit_point_vector));

        if !Self::is_interior(alpha, beta) {
            return false;
        }

        // Ray hits the 2D shape; set the rest of the hit record and return true.
        hit_record.u = alpha;
        hit_record.v = beta;
        hit_record.t = t;
        hit_record.point = intersection;
        hit_record.material = self.material.clone();
        hit_record.set_face_normal(&ray, &self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bounding_box
    }
}

impl Quad {
    fn is_interior(alpha: f32, beta: f32) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        unit_interval.contain(alpha) && unit_interval.contain(beta)
    }

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

impl Quad {
    pub fn new(origin: Point3, u: Vec3, v: Vec3, material: Rc<dyn Material>) -> Self {
        let bounding_box = Self::get_new_bounding_box(origin, u, v);
        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(origin);
        let w = n / n.dot(n);
        Self {
            origin,
            u,
            v,
            w,
            material,
            bounding_box,
            normal,
            d,
        }
    }
}
