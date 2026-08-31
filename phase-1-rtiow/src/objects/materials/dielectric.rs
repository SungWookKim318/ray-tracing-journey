use crate::{color::Color, ray::Ray};

use super::material::Material;

pub struct Dielectric {
    refraction_ratio: f32, // Air to Matter's eta ratio (reflective ratio)
}

impl Dielectric {
    pub fn new(refraction_ratio: f32) -> Self {
        Self { refraction_ratio }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray_in: crate::ray::Ray,
        record: &mut crate::objects::hittable::HitRecord,
        attenuation: &mut crate::color::Color,
        scattered: &mut crate::ray::Ray,
        _: &mut dyn rand::Rng,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let faced_reflacted_ratio = if record.is_front_face {
            1.0 / self.refraction_ratio
        } else {
            self.refraction_ratio
        };

        let unit_direction = ray_in.direction().normalize();
        let refracted = unit_direction.refract(record.normal, faced_reflacted_ratio);

        *scattered = Ray::new(record.point, refracted);
        true
    }
}
