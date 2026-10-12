use std::rc::Rc;

use rand::Rng;

use crate::{
    color::Color,
    objects::{
        hittabbles::hittable::HitRecord,
        materials::material::Material,
        textures::{solid_texture::SolidTexture, texture::Texture},
    },
    ray::Ray,
    vec3::Vec3,
};

pub struct Isotropic {
    texture: Rc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray_in: Ray,
        record: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        rng: &mut dyn Rng,
    ) -> bool {
        *scattered = Ray::new(record.point, Vec3::random_sphere(rng), ray_in.time());
        *attenuation = self.texture.value(record.u, record.v, &record.point);
        true
    }

    fn emitted(&self, _: f32, _: f32, _: &crate::vec3::Point3) -> Color {
        Color::zero()
    }
}

impl Isotropic {
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
