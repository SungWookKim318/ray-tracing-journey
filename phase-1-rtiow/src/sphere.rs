use crate::Point3;
use crate::Ray;

pub fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> bool {
    let origin_center = *center - ray.origin();
    let a = ray.direction().dot(ray.direction());
    let b = -2.0 * ray.direction().dot(origin_center);
    let c = origin_center.dot(origin_center) - (radius * radius);
    let discriminant = b * b - (4.0 * a * c);
    discriminant >= 0.0
}
