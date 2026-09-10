use crate::{color::Color, objects::texture::solid_texture::SolidTexture};

use super::texture::Texture;
use std::rc::Rc;

pub struct CheckerTexture {
    cell_per_unit: f32,
    even_texture: Rc<dyn Texture>,
    odd_texture: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new_(scale: f32, even_texture: Rc<dyn Texture>, odd_texture: Rc<dyn Texture>) -> Self {
        assert!(scale > 0.0, "scale should be bigger than zero.");
        Self {
            cell_per_unit: 1.0 / scale,
            even_texture,
            odd_texture,
        }
    }

    pub fn new_with_color(scale: f32, even_color: Color, odd_color: Color) -> Self {
        Self::new_(
            scale,
            Rc::new(SolidTexture::new(even_color)),
            Rc::new(SolidTexture::new(odd_color)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: &crate::vec3::Point3) -> Color {
        let x = (self.cell_per_unit * point.x).floor() as i32;
        let y = (self.cell_per_unit * point.y).floor() as i32;
        let z = (self.cell_per_unit * point.z).floor() as i32;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even_texture.value(u, v, point)
        } else {
            self.odd_texture.value(u, v, point)
        }
    }
}
