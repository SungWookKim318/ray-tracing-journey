use super::texture::Texture;
use crate::color::Color;

pub struct SolidTexture {
    albedo: Color,
}

impl SolidTexture {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn albedo(&self) -> Color {
        self.albedo
    }
}

impl Texture for SolidTexture {
    fn value(&self, _: f32, _: f32, _: &crate::vec3::Point3) -> Color {
        self.albedo
    }
}
