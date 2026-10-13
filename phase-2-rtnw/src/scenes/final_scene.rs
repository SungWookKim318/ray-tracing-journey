use core::panic;
use std::rc::Rc;

use itertools::iproduct;

use crate::{
    color::Color,
    objects::{
        bvh::BvhNode,
        camera::Camera,
        hittabbles::{
            constant_medium::ConstantMedium, hittable_list::HittableList, quad::Quad,
            rotate_y::RotateY, sphere::Sphere, translate::Translate,
        },
        materials::{
            dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian,
            metal::Metal,
        },
        textures::{
            image_texture::{ImageTexture, PixelType},
            noise_texture::NoiseTexture,
        },
    },
    utils::noises::turbulence_perlin::TurbulencePerlin,
    vec3::{Point3, Vec3},
};

pub fn final_scene(image_width: i32, sample_per_pixel: i32, max_depth: i32) {
    eprintln!(
        "start final_scene({}, {}, {})",
        image_width, sample_per_pixel, max_depth
    );
    let mut rng = rand::rng();
    let mut boxes1 = HittableList::new();

    let ground_mat = Rc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    let boxes_per_side = 20;
    //for (i, j, k) in iproduct!(0..1, 0..1, 0..1).map(|(i, j, k)| (i as f32, j as f32, k as f32))
    for (i, j) in iproduct!(0..boxes_per_side, 0..boxes_per_side).map(|(i, j)| (i as f32, j as f32))
    {
        let w = 100.0;
        let x0 = -1000.0 + i * w;
        let z0 = -1000.0 + j * w;
        let y0 = 0.0;

        let x1 = x0 + w;
        let y1 = rand::random_range(1.0..=101.0);
        let z1 = z0 + w;

        let point0 = Point3::new(x0, y0, z1);
        let point1 = Point3::new(x1, y1, z1);

        boxes1.add(Rc::new(Quad::make_box(point0, point1, ground_mat.clone())));
    }

    let mut world = HittableList::new();
    let bvh_boxes1 = BvhNode::new_root(&mut boxes1);
    world.add(Rc::new(bvh_boxes1));

    let light = Rc::new(DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Rc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center_point_1 = Point3::new(400.0, 400.0, 200.0);
    let center_point_2 = center_point_1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_mat = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Rc::new(Sphere::moved_new(
        center_point_1,
        center_point_2,
        50.0,
        sphere_mat,
    )));

    world.add(Rc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Rc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Rc::new(ConstantMedium::new(
        boundary.clone(),
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    world.add(Rc::new(ConstantMedium::new(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let image_path = "assets/earthmap.jpg";
    let earth_texture = match ImageTexture::load_file(image_path.to_string(), PixelType::RGB888) {
        Some(texture) => Rc::new(texture),
        None => {
            panic!("fail to load image");
        }
    };
    let earth_mat = Rc::new(Lambertian::new_with_texture(earth_texture));
    world.add(Rc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_mat,
    )));
    let perlin_texture = Rc::new(NoiseTexture::new(
        Box::new(TurbulencePerlin::new(&mut rng)),
        4.0,
    ));
    let noise_mat = Rc::new(Lambertian::new_with_texture(perlin_texture));
    world.add(Rc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        noise_mat,
    )));

    let mut boxes2 = HittableList::new();
    let white_mat = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    for _ in 0..1000 {
        boxes2.add(Rc::new(Sphere::new(
            Point3::random_range(&mut rng, 0.0, 165.0),
            10.0,
            white_mat.clone(),
        )));
    }

    let bvh_boxes2 = BvhNode::new_root(&mut boxes2);
    world.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(Rc::new(bvh_boxes2), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let mut camera = Camera::zero();
    camera.aspect_ratio = 1.0;
    camera.image_width = image_width;
    camera.sample_per_pixel = sample_per_pixel;
    camera.max_depth = max_depth;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.vertical_fov = 40.0;
    camera.center = Point3::new(478.0, 278.0, -600.0);
    camera.look_at = Point3::new(278.0, 278.0, 0.0);
    camera.up_direction = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    let bvh_world = BvhNode::new_root(&mut world);
    let ray_generate_time = std::time::Instant::now();
    camera.render(&bvh_world);
    eprintln!("End final_scene. {:.2?}", ray_generate_time.elapsed());
}
