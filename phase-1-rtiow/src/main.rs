#![allow(dead_code)]
mod color;
mod imagedata;
mod objects;
mod ray;
mod utils;
mod vec3;

use crate::objects::{
    camera::Camera,
    hittable_list,
    materials::{dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal},
    sphere::Sphere,
};

use color::Color;
use rand::RngExt;
use ray::Ray;
use std::rc::Rc;
use vec3::{Point3, Vec3};

fn main() {
    eprintln!("Start RT");
    let mut rng = rand::rng();
    // World
    let mut world = hittable_list::HittableList::new();
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::clone(&ground_material),
    )));

    for x_object in -11..=11 {
        for z_object in -11..=11 {
            let percentage = rng.random_range(0.0..=1.0);
            let object_position = Vec3::new(
                (x_object as f32) + 0.9 * rng.random_range(0.0..=1.0),
                0.2,
                (z_object as f32) + 0.9 * rng.random_range(0.0..=1.0),
            );
            if percentage < 0.8 {
                // Lambertian
                let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                let new_material: Rc<dyn Material> = Rc::new(Lambertian::new(albedo));
                world.add(Rc::new(Sphere::new(
                    object_position,
                    0.2,
                    Rc::clone(&new_material),
                )));
            } else if percentage < 0.95 {
                // metal
                let albedo = Color::random_range(&mut rng, 0.6, 1.0);
                let fuzz = rng.random_range(0.0..=0.5);
                let new_material: Rc<dyn Material> = Rc::new(Metal::new(albedo, fuzz));
                world.add(Rc::new(Sphere::new(
                    object_position,
                    0.2,
                    Rc::clone(&new_material),
                )));
            } else {
                // glass
                // metal
                let new_material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
                world.add(Rc::new(Sphere::new(
                    object_position,
                    0.2,
                    Rc::clone(&new_material),
                )));
            };
        }
    }

    let left_material: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Rc::clone(&left_material),
    )));

    let center_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&center_material),
    )));

    let right_material: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Rc::clone(&right_material),
    )));

    let mut camera = Camera::zero();
    camera.aspect_ratio = 16.0 / 9.0f32;
    camera.image_width = 400;
    camera.sample_per_pixel = 200;
    camera.max_depth = 50;

    camera.vertical_fov = 20.0;
    camera.center = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.up_direction = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    camera.render(&world);
    eprintln!("End RT");
}
