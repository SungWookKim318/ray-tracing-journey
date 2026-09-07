use super::material::Material;
use crate::{color::Color, objects::hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray: Ray,
        record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn rand::Rng,
    ) -> bool {
        let mut scatter_direction = record.normal + Vec3::random_sphere(rng);
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal;
        }
        *scattered = Ray::new(record.point, scatter_direction, ray.time());
        *attenuation = self.albedo;
        true
    }
}
