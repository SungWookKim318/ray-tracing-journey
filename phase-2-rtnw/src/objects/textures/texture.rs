use crate::{color::Color, vec3::Point3};

pub trait Texture {
    fn value(&self, u: f32, v: f32, point: &Point3) -> Color;
}
