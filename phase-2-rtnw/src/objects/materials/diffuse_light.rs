use std::rc::Rc;

use crate::{
    color::Color,
    objects::{
        hittable::HitRecord,
        textures::{solid_texture::SolidTexture, texture::Texture},
    },
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

impl DiffuseLight {
    pub fn new_with_color(color: Color) -> Self {
        let new_texture = Rc::new(SolidTexture::new(color));
        Self {
            texture: new_texture,
        }
    }
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { texture: albedo }
    }
}
