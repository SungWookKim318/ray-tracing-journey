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
    materials::material::{Material, NoneMaterial},
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
    let none_material: Rc<dyn Material> = Rc::new(NoneMaterial::new());

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&none_material),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&none_material),
    )));

    let mut camera = Camera::zero();
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;
    camera.aspect_ratio = 16.0 / 9.0f32;
    camera.image_width = 400;

    camera.render(&world);
    eprintln!("End RT");
}
