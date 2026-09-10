use crate::objects::{
    bvh::BvhNode,
    camera::Camera,
    hittable_list,
    materials::{dielectric::Dielectric, lambertian::Lambertian, material::Material, metal::Metal},
    sphere::Sphere,
    textures::{checker_texture::CheckerTexture, texture::Texture},
};

use crate::color::Color;
use crate::vec3::{Point3, Vec3};
use rand::RngExt;
use std::rc::Rc;

pub(crate) fn bouncing_spheres_with_bvh() {
    eprintln!("Start bouncing_spheres_with_bvh");
    let mut rng = rand::rng();
    // World
    let mut world = hittable_list::HittableList::new();
    let ground_texture: Rc<dyn Texture> = Rc::new(CheckerTexture::new_with_color(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material: Rc<dyn Material> = Rc::new(Lambertian::new_with_texture(ground_texture));
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
                let moved_position =
                    object_position + Vec3::new(0.0, rng.random_range(0.0..=0.5), 0.0);
                world.add(Rc::new(Sphere::moved_new(
                    object_position,
                    moved_position,
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

    eprintln!("Create BVH");
    let bvh_start_time = std::time::Instant::now();
    let bvh_world = BvhNode::new_root(&mut world);
    eprintln!(
        "Finished BVH, Start Generate Rays. {:.2?}",
        bvh_start_time.elapsed()
    );

    let mut camera = Camera::zero();
    camera.aspect_ratio = 16.0 / 9.0f32;
    camera.image_width = 400;
    camera.sample_per_pixel = 50;
    camera.max_depth = 20;

    camera.vertical_fov = 20.0;
    camera.center = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.up_direction = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let ray_generate_time = std::time::Instant::now();
    camera.render(&bvh_world);
    eprintln!("End RT with BVH. {:.2?}", ray_generate_time.elapsed());
}
