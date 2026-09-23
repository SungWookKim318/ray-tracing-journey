use std::{eprint, rc::Rc};

use crate::{
    color::Color,
    objects::{
        camera::Camera,
        hittable_list::HittableList,
        materials::{lambertian::Lambertian, material::Material},
        quad::Quad,
    },
    vec3::{Point3, Vec3},
};

pub(crate) fn basic_quads() {
    eprint!("start basic_quads");
    // World
    let mut world = HittableList::new();
    let left_red_color = Color::new(1.0, 0.2, 0.2);
    let back_green_color = Color::new(0.2, 1.0, 0.2);
    let right_blue_color = Color::new(0.2, 0.2, 1.0);
    let upper_orange_color = Color::new(1.0, 0.5, 0.0);
    let lower_teal_color = Color::new(0.2, 0.8, 0.8);

    let left_red_material: Rc<dyn Material> = Rc::new(Lambertian::new(left_red_color));
    let back_green_material: Rc<dyn Material> = Rc::new(Lambertian::new(back_green_color));
    let right_blue_material: Rc<dyn Material> = Rc::new(Lambertian::new(right_blue_color));
    let upper_orange_material: Rc<dyn Material> = Rc::new(Lambertian::new(upper_orange_color));
    let lower_teal_material: Rc<dyn Material> = Rc::new(Lambertian::new(lower_teal_color));

    world.add(Rc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red_material,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green_material,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue_material,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange_material,
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal_material,
    )));
    let mut camera = Camera::zero();
    camera.aspect_ratio = 1.0;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;

    camera.vertical_fov = 80.0;
    camera.center = Point3::new(0.0, 0.0, 9.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.up_direction = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    let ray_generate_time = std::time::Instant::now();
    camera.render(&world);
    eprintln!("End basic_quads. {:.2?}", ray_generate_time.elapsed());
}
