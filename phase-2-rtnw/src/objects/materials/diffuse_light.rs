use std::rc::Rc;

use crate::{
    color::Color,
    objects::{hittable::HitRecord, textures::texture::Texture},
    ray::Ray,
    vec3::Point3,
};

use super::material::Material;

pub struct DiffuseLight {
    texture: Rc<dyn Texture>,
}

impl Material for DiffuseLight {
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

    fn emitted(&self, u: f32, v: f32, point: &Point3) -> Color {
        self.texture.value(u, v, point)
    }
}
