use rand::RngExt;

use crate::{
    color::Color,
    image_data::ImageData,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::interval::Interval,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub sample_per_pixel: i32,
    pub max_depth: i32,
    pub image_width: i32,
    pub vertical_fov: f32,
    pub center: Point3,
    pub look_at: Point3,
    pub up_direction: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,

    image_height: i32,
    pixel_sample_scale: f32,
    pixel_origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    basis_u: Vec3,
    basis_v: Vec3,
    basis_w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}
// Constrcutor
impl Camera {
    pub const fn zero() -> Self {
        Self {
            aspect_ratio: 0.0,
            sample_per_pixel: 0,
            max_depth: 10,
            vertical_fov: 0.0,
            center: Vec3::zero(),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            up_direction: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_width: 0,
            image_height: 0,
            pixel_sample_scale: 0.0,
            pixel_origin: Vec3::zero(),
            pixel_delta_u: Vec3::zero(),
            pixel_delta_v: Vec3::zero(),
            basis_u: Vec3::zero(),
            basis_v: Vec3::zero(),
            basis_w: Vec3::zero(),
            defocus_disk_u: Vec3::zero(),
            defocus_disk_v: Vec3::zero(),
        }
    }
}
impl Camera {
    // Publics
    pub fn render(&mut self, world: &dyn Hittable) {
        self.setup();

        let mut image = ImageData::new(self.image_width as usize, self.image_height as usize);
        let mut rng = rand::rng();
        for (index, pixel) in image.data.iter_mut().enumerate() {
            let x = index as i32 % self.image_width;
            let y = index as i32 / self.image_width;

            for _ in 0..self.sample_per_pixel {
                let sample_ray = self.get_ray(x, y, &mut rng);
                *pixel += self.ray_color(sample_ray, world, self.max_depth, &mut rng);
            }
            *pixel *= self.pixel_sample_scale;
        }

        image.print_to_ppm();
    }

    // Private
    fn setup(&mut self) {
        assert!(self.aspect_ratio > 0.0, "Aspect Ratio is under Zero.");
        assert!(self.image_width > 0, "Image width is under Zero.");
        assert!(self.sample_per_pixel > 0, "Simple per pixel is under Zero.");
        assert!(self.max_depth > 0, "max_depth is under Zero.");
        assert!(self.vertical_fov > 0.0, "Vertical FOV is under Zero.");

        self.pixel_sample_scale = 1.0 / self.sample_per_pixel as f32;
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        // Determine viewport dimensions.
        let half_theta_radian = self.vertical_fov.to_radians() / 2.0;
        let height_fov = (half_theta_radian).tan();
        let viewport_height = 2.0 * height_fov * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        self.basis_w = (self.center - self.look_at).normalize();
        self.basis_u = self.up_direction.cross(self.basis_w).normalize();
        self.basis_v = self.basis_w.cross(self.basis_u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * self.basis_u;
        let viewport_v = -viewport_height * self.basis_v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        // Calculate the location of the upper left pxiel(origin)
        let viewport_upper_left =
            self.center - (self.focus_dist * self.basis_w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_origin = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle.to_radians() / 2.0).tan();
        self.defocus_disk_u = self.basis_u * defocus_radius;
        self.defocus_disk_v = self.basis_v * defocus_radius;
    }

    fn get_ray(&self, x: i32, y: i32, rng: &mut dyn rand::Rng) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location x, y.
        let offset = Camera::sample_square(rng);
        let pixel_sample = self.pixel_origin
            + ((x as f32 + offset.x) * self.pixel_delta_u)
            + ((y as f32 + offset.y) * self.pixel_delta_v);
        // let ray_origin = self.center;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(rng: &mut dyn rand::Rng) -> Vec3 {
        Vec3::new(
            rng.random_range(0.0..=1.0) - 0.5,
            rng.random_range(0.0..=1.0) - 0.5,
            0.0,
        )
    }

    fn ray_color(
        &self,
        ray: Ray,
        world: &dyn Hittable,
        depth: i32,
        rng: &mut dyn rand::Rng,
    ) -> Color {
        if depth <= 0 {
            return Color::zero();
        }

        let mut record = HitRecord::new();
        if world.hit(ray, Interval::new(0.001, f32::INFINITY), &mut record) {
            let mut scattered_ray = Ray::zero();
            let mut attenuation = Vec3::zero();
            let material = record.material.clone();
            if material.scatter(ray, &mut record, &mut attenuation, &mut scattered_ray, rng) {
                return attenuation * self.ray_color(scattered_ray, world, depth - 1, rng);
            }
            return Color::zero();
        }
        let unit_direction = ray.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::with_scalar(1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn defocus_disk_sample(&self, rng: &mut dyn rand::Rng) -> Point3 {
        let rand_point = Vec3::random_unit_disk(rng);
        self.center + self.defocus_disk_u * rand_point.x + self.defocus_disk_v * rand_point.y
    }
}
