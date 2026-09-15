use std::{eprint, rc::Rc};

use crate::{
    objects::{
        camera::Camera,
        hittable_list::HittableList,
        materials::{lambertian::Lambertian, material::Material},
        sphere::Sphere,
        textures::noise_texture::NoiseTexture,
    },
    vec3::{Point3, Vec3},
};

pub(crate) fn perlin_spheres() {
    eprint!("start perlin_spheres");
    let mut rng = rand::rng();
    let mut world = HittableList::new();

    let perlin_texture = Rc::new(NoiseTexture::new(&mut rng));
    let surface: Rc<dyn Material> = Rc::new(Lambertian::new_with_texture(perlin_texture));
    let ground = Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        surface.clone(),
    ));
    world.add(ground);
    let globe = Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        surface.clone(),
    ));
    world.add(globe);

    let mut camera = Camera::zero();
    camera.aspect_ratio = 16.0 / 9.0;
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
    eprintln!("End perlin_spheres. {:.2?}", ray_generate_time.elapsed());
}
