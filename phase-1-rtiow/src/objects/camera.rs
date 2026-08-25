use crate::{
    color::Color,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::interval::Interval,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    center: Point3,
    pixel_origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}
// Constrcutor
impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        let center = Point3::zero();

        // Determine viewport dimensions.
        let focal_length = 1.0f32;
        let viewport_height = 2.0f32;
        let viewport_width = viewport_height * (image_height as f32 / image_height as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Claculate the location of the upper left pxiel(origin)
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel_origin,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
impl Camera {
    // Publics
    pub fn render(world: &dyn Hittable) {}

    // Private
    fn initilize() {}
    fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
        let mut record = HitRecord::new();
        if world.hit(ray, Interval::zero_to_inf(), &mut record) {
            return 0.5 * (record.normal + Color::with_scalar(1.0));
        }
        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::with_scalar(1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
