use std::eprintln;

use image::{DynamicImage, GenericImageView, Pixel};

use crate::{
    color::Color, objects::textures::texture::Texture, utils::interval::Interval, vec3::Point3,
};

pub struct ImageTexture {
    image: DynamicImage,
    pixel_type: PixelType,
}

pub enum PixelType {
    RGB888,
    RGBA8888,
}

impl ImageTexture {
    pub fn loadFile(path: String, pixel_type: PixelType) -> Option<Self> {
        let image = match image::open(path) {
            Ok(data) => data,
            Err(error) => {
                eprintln!("fail to open image path. please check it. {}", error);
                return Option::None;
            }
        };
        Option::Some(Self { image, pixel_type })
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _: &Point3) -> Color {
        if self.image.height() <= 0 || self.image.width() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }
        let interval = Interval::new(0.0, 1.0);
        let clamped_u = interval.clamp(u);
        let clamped_v = 1.0 - interval.clamp(v);

        let pixel_x = (clamped_u * self.image.width() as f32) as u32;
        let pixel_y = (clamped_v * self.image.height() as f32) as u32;
        let pixel = self.image.get_pixel(pixel_x, pixel_y).to_rgb();

        Color::new(pixel[0] as f32, pixel[1] as f32, pixel[2] as f32) / 255.0
    }
}
