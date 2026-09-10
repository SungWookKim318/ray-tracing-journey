use std::rc::Rc;

use super::material::Material;
use crate::{
    color::Color,
    objects::{
        hittable::HitRecord,
        textures::{solid_texture::SolidTexture, texture::Texture},
    },
    ray::Ray,
    vec3::Vec3,
};

pub struct Lambertian {
    texture: Rc<dyn Texture>,
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        let new_texture: Rc<dyn Texture> = Rc::new(SolidTexture::new(color));
        Self {
            texture: new_texture,
        }
    }
    pub fn new_with_texture(albedo: Rc<dyn Texture>) -> Self {
        Self { texture: albedo }
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
        *attenuation = self.texture.value(record.u, record.v, &record.point);
        true
    }
}
