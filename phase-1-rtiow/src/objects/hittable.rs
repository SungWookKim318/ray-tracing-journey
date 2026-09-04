use std::rc::Rc;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::objects::materials::material::Material;
use crate::objects::materials::material::NoneMaterial;
use crate::utils::interval::Interval;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            is_front_face: false,
            material: Rc::new(NoneMaterial::new()),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.is_front_face = ray.direction().dot(*outward_normal) < 0.0;
        self.normal = if self.is_front_face {
            *outward_normal
        } else {
            -outward_normal
        }
        //self.is_front_face ? *outward_normal : -*outward_normal;
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool;
}
