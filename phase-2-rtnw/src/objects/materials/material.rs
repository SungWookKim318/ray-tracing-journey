use crate::{color::Color, objects::hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(
        &self,
        ray_in: Ray,
        record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn rand::Rng,
    ) -> bool;
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
}

impl NoneMaterial {
    pub fn new() -> Self {
        Self {}
    }
}
