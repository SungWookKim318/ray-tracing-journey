#![allow(dead_code)]
mod color;
mod imagedata;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use imagedata::ImageData;
use ray::Ray;
use vec3::Point3;
use vec3::Vec3;

use crate::sphere::hit_sphere;

fn main() {
    eprintln!("Start RT");

    // setup canvas
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * ((image_width as f64) / (image_height as f64));
    let camera_center = Point3::zero();

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width as f32, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height as f32, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / (image_width as f32);
    let pixel_delta_v = viewport_v / (image_height as f32);

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut image: ImageData = ImageData::new(image_width as usize, image_height as usize);

    // Renderring
    for (index, pixel) in image.data.iter_mut().enumerate() {
        let x = (index % (image_width as usize)) as f32;
        let y = (index / (image_width as usize)) as f32;

        let pixel_center = pixel00_loc + (x * pixel_delta_u) + (y * pixel_delta_v);
        let ray_direction = pixel_center - camera_center;
        let ray = Ray::new(camera_center, ray_direction);
        *pixel = ray_color(ray);
    }

    image.print_to_ppm();
}

fn ray_color(ray: Ray) -> Color {
    let center = Point3::new(0.0, 0.0, -1.0);
    if hit_sphere(&center, 0.5, &ray) {
        return 255.999 * Color::new(1., 0., 0.);
    }

    let unit_direction = ray.direction().normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    let norm_color = (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0);
    255.999 * norm_color
}
