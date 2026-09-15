use crate::vec3::Point3;

pub trait Noise {
    fn noise(&self, position: Point3) -> f32;
}
