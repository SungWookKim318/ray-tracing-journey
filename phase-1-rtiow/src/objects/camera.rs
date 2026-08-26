use crate::{
    color::Color,
    imagedata::ImageData,
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
    pub const fn zero() -> Self {
        Self {
            aspect_ratio: 0.0,
            image_width: 0,
            image_height: 0,
            center: Vec3::zero(),
            pixel_origin: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
        }
    }
}
impl Camera {
    // Publics
    pub fn render(&mut self, world: &dyn Hittable) {
        self.setup();

        let mut image = ImageData::new(self.image_width as usize, self.image_height as usize);

        for (index, pixel) in image.data.iter_mut().enumerate() {
            let x = (index % (self.image_width as usize)) as f32;
            let y = (index / (self.image_width as usize)) as f32;

            let pixel_center =
                self.pixel_origin + (x * self.pixel_delta_u) + (y * self.pixel_delta_v);
            let ray_direction = pixel_center - self.center;
            let ray = Ray::new(self.center, ray_direction);
            *pixel = 255.999 * self.ray_color(ray, world);
        }

        image.print_to_ppm();
    }

    // Private
    fn setup(&mut self) {
        assert!(
            self.aspect_ratio > 0.0 || self.image_width > 0,
            "Aspect Ratio and Image width is under Zero."
        );

        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.center = Point3::zero();

        // Determine viewport dimensions.
        let focal_length = 1.0f32;
        let viewport_height = 2.0f32;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Claculate the location of the upper left pxiel(origin)
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_origin = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, ray: Ray, world: &dyn Hittable) -> Color {
        let mut record = HitRecord::new();
        if world.hit(ray, Interval::zero_to_inf(), &mut record) {
            return 0.5 * (record.normal + Color::with_scalar(1.0));
        }
        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::with_scalar(1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
