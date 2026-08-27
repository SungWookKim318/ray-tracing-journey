use crate::{utils::interval::Interval, vec3::Vec3};

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) {
        let intensity = Interval::new(0.0, 0.999);

        let r = (256.0 * intensity.clamp(self.x)) as i32;
        let g = (256.0 * intensity.clamp(self.y)) as i32;
        let b = (256.0 * intensity.clamp(self.z)) as i32;

        print!("{} {} {} ", r, g, b,);
    }
}
