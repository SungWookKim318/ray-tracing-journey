use crate::{
    color::Color,
    utils::noises::{noise::Noise, perlin::Perlin},
    vec3::{Point3, Vec3},
};

use super::texture::Texture;

pub struct NoiseTexture {
    noise: Box<dyn Noise>,
}

impl NoiseTexture {
    pub fn new(noise: Box<dyn Noise>) -> Self {
        Self { noise }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f32, _: f32, point: &Point3) -> Vec3 {
        Color::with_scalar(1.0) * self.noise.noise(*point)
    }
}
