use crate::{color::Color, objects::hittabbles::hittable::HitRecord, ray::Ray, vec3::Point3};

pub trait Material {
    fn scatter(
        &self,
        ray_in: Ray,
        record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn rand::Rng,
    ) -> bool;

    fn emitted(&self, _: f32, _: f32, _: &Point3) -> Color;
}

pub struct NoneMaterial {}
impl Material for NoneMaterial {
    fn scatter(
        &self,
        _: Ray,
        _: &mut HitRecord,
        _: &mut Color,
        _: &mut Ray,
        _: &mut dyn rand::Rng,
    ) -> bool {
        false
    }

    fn emitted(&self, _: f32, _: f32, _: &Point3) -> Color {
        Color::zero()
    }
}

impl NoneMaterial {
    pub fn new() -> Self {
        Self {}
    }
}
