use crate::Point3;
use crate::Ray;

pub fn hit_sphere(center: &Point3, radius: f32, ray: Ray) -> f32 {
    let origin_center = *center - ray.origin();

    let a = ray.direction().length_squared();
    let h = ray.direction().dot(origin_center);
    let c = origin_center.length_squared() - (radius * radius);
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}
