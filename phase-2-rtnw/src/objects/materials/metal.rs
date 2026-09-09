use super::material::Material;
use crate::{color::Color, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: crate::ray::Ray,
        record: &mut crate::objects::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
        rng: &mut dyn rand::Rng,
    ) -> bool {
        let reflected = ray_in.direction().reflect(record.normal);
        let fuzzed_reflected = reflected.normalize() + (self.fuzz * Vec3::random_sphere(rng));
        *scattered = Ray::new(record.point, fuzzed_reflected, ray_in.time());
        *attenuation = self.albedo;

        scattered.direction().dot(record.normal) > 0.0
    }
}
