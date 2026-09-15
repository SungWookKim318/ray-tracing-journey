use crate::{
    color::Color,
    utils::noises::perlin::Perlin,
    vec3::{Point3, Vec3},
};

use super::texture::Texture;

#[derive(Debug)]
pub struct NoiseTexture {
    noise: Perlin,
}

impl NoiseTexture {
    pub fn new(rng: &mut dyn rand::Rng) -> Self {
        Self {
            noise: Perlin::new(rng),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _: f32, _: f32, point: &Point3) -> Vec3 {
        Color::with_scalar(1.0) * self.noise.noise(*point)
    }
}
