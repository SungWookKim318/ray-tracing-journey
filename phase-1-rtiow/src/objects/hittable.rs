use crate::Point3;
use crate::Ray;
use crate::Vec3;

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool,
}

impl HitRecord {
    pub const fn new() -> Self {
        Self {
            point: Vec3::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            is_front_face: false,
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
    fn hit(&self, ray: Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool;
}
