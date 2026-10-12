use std::rc::Rc;

use itertools::iproduct;

use crate::{
    color::Color,
    objects::{
        bvh::BvhNode,
        camera::Camera,
        hittabbles::{hittable_list::HittableList, quad::Quad},
        materials::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    },
    vec3::{Point3, Vec3},
};

fn final_scene(image_width: i32, sample_per_pixel: i32, max_depth: i32) {
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
    world.add(Rc::new(boxes1));

    let light = Rc::new(DiffuseLight::new_with_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Rc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
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

    eprintln!("Create BVH");
    let bvh_start_time = std::time::Instant::now();
    let bvh_world = BvhNode::new_root(&mut world);
    eprintln!(
        "Finished BVH, Start Generate Rays. {:.2?}",
        bvh_start_time.elapsed()
    );

    let ray_generate_time = std::time::Instant::now();
    camera.render(&bvh_world);
    eprintln!("End cornell_smoke. {:.2?}", ray_generate_time.elapsed());
}
