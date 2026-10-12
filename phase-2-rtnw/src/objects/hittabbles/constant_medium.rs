use std::rc::Rc;

use rand::random_range;

use crate::{
    color::Color,
    objects::{
        hittabbles::hittable::{HitRecord, Hittable},
        materials::{isotropic::Isotropic, material::Material},
        textures::{solid_texture::SolidTexture, texture::Texture},
    },
    ray::Ray,
    utils::interval::Interval,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    negitive_inverse_density: f32,
    phase_function: Rc<dyn Material>,
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: Ray, ray_t: Interval, hit_record: &mut HitRecord) -> bool {
        let mut record_1 = HitRecord::new();

        if !self.boundary.hit(ray, Interval::universe(), &mut record_1) {
            return false;
        }

        let mut record_2 = HitRecord::new();
        if !self.boundary.hit(
            ray,
            Interval::new(record_1.t + 0.0001, f32::INFINITY),
            &mut record_2,
        ) {
            return false;
        }

        if record_1.t < ray_t.min {
            record_1.t = ray_t.min
        }
        if record_2.t > ray_t.max {
            record_2.t = ray_t.max
        }

        if record_1.t >= record_2.t {
            return false;
        }
        if record_1.t < 0.0 {
            record_1.t = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (record_2.t - record_1.t) * ray_length;
        let hit_distance = self.negitive_inverse_density * random_range(0.0..=1.0f32).ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        hit_record.t = record_1.t + hit_distance / ray_length;
        hit_record.point = ray.at(hit_record.t);

        hit_record.normal = Vec3::new(1.0, 0.0, 0.0);
        hit_record.is_front_face = true;
        hit_record.material = self.phase_function.clone();

        true
    }

    fn bounding_box(&self) -> crate::utils::aabb::Aabb {
        self.boundary.bounding_box()
    }
}

impl ConstantMedium {
    pub fn new_with_texture(
        boundary: Rc<dyn Hittable>,
        density: f32,
        texture: Rc<dyn Texture>,
    ) -> Self {
        let new_isotropic = Isotropic::new(texture);
        Self {
            boundary,
            negitive_inverse_density: -1.0 / density,
            phase_function: Rc::new(new_isotropic),
        }
    }

    pub fn new(boundary: Rc<dyn Hittable>, density: f32, albedo: Color) -> Self {
        let texture = SolidTexture::new(albedo);
        let new_isotropic = Isotropic::new(Rc::new(texture));
        Self {
            boundary,
            negitive_inverse_density: -1.0 / density,
            phase_function: Rc::new(new_isotropic),
        }
    }
}
