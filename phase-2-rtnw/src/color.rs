use crate::{utils::interval::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    fn linear_to_gamma(color: f32) -> f32 {
        if color > 0.0 { f32::sqrt(color) } else { color }
    }

    pub fn write_color(&self) {
        let intensity = Interval::new(0.0, 0.999);
        let gamma_x = Color::linear_to_gamma(self.x);
        let gamma_y = Color::linear_to_gamma(self.y);
        let gamma_z = Color::linear_to_gamma(self.z);

        let r = 256.0 * intensity.clamp(gamma_x);
        let g = 256.0 * intensity.clamp(gamma_y);
        let b = 256.0 * intensity.clamp(gamma_z);

        print!("{} {} {} ", r as i32, g as i32, b as i32);
    }
}
