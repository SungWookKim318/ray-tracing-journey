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
use ray::Ray;
use std::rc::Rc;
use vec3::{Point3, Vec3};

fn main() {
    eprintln!("Start RT");

    // World
    let mut world = hittable_list::HittableList::new();
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let center_material: Rc<dyn Material> = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let left_material: Rc<dyn Material> = Rc::new(Dielectric::new(1.0 / 1.33));
    let right_material: Rc<dyn Material> = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&ground_material),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        Rc::clone(&center_material),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&left_material),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&right_material),
    )));

    let mut camera = Camera::zero();
    camera.aspect_ratio = 16.0 / 9.0f32;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;

    camera.render(&world);
    eprintln!("End RT");
}
