use std::rc::Rc;

use crate::{
    color::Color,
    objects::{
        camera::Camera,
        hittable_list::HittableList,
        materials::lambertian::Lambertian,
        sphere::Sphere,
        textures::{checker_texture::CheckerTexture, texture::Texture},
    },
    vec3::{Point3, Vec3},
};

pub(crate) fn checkered_spheres() {
    eprintln!("Start checkered_spheres");
    let mut world = HittableList::new();

    let checker_texture: Rc<dyn Texture> = Rc::new(CheckerTexture::new_with_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let material = Rc::new(Lambertian::new_with_texture(checker_texture));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        material.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        material.clone(),
    )));

    let mut camera = Camera::zero();
    camera.aspect_ratio = 16.0 / 9.0f32;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;

    camera.vertical_fov = 20.0;
    camera.center = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.up_direction = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    let ray_generate_time = std::time::Instant::now();
    camera.render(&world);
    eprintln!("End checkered_spheres. {:.2?}", ray_generate_time.elapsed());
}
