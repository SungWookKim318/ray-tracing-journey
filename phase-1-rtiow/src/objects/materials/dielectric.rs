use crate::{color::Color, ray::Ray};

use super::material::Material;

pub struct Dielectric {
    refractive_index: f32, // Air to Matter's eta ratio (reflective ratio)
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
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
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction().normalize();

        let cos_theta = unit_direction.dot(-record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if faced_reflacted_ratio * sin_theta > 1.0 {
            unit_direction.reflect(record.normal)
        } else {
            unit_direction.refract(record.normal, faced_reflacted_ratio)
        };

        *scattered = Ray::new(record.point, direction);
        true
    }
}
