use super::material::Material;
use crate::{color::Color, ray::Ray};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: crate::ray::Ray,
        record: &mut crate::objects::hittable::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::ray::Ray,
        _: &mut dyn rand::Rng,
    ) -> bool {
        let reflected = ray_in.direction().refelct(record.normal);
        *scattered = Ray::new(record.point, reflected);
        *attenuation = self.albedo;

        true
    }
}
