use crate::{
    color::Color,
    objects::{
        camera::Camera,
        hittabbles::{hittable_list::HittableList, quad::Quad, sphere::Sphere},
        materials::{diffuse_light::DiffuseLight, lambertian::Lambertian, material::Material},
        textures::{noise_texture::NoiseTexture, texture::Texture},
    },
    utils::noises::turbulence_perlin::TurbulencePerlin,
    vec3::{Point3, Vec3},
};
use std::rc::Rc;

pub fn simple_light() {
    eprint!("start simple_light");
    let mut rng = rand::rng();
    let mut world = HittableList::new();

    let perlin_texture: Rc<dyn Texture> = Rc::new(NoiseTexture::new(
        Box::new(TurbulencePerlin::new(&mut rng)),
        4.0,
    ));
    let surface: Rc<dyn Material> = Rc::new(Lambertian::new_with_texture(perlin_texture));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        surface.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        surface.clone(),
    )));

    let diffuse_light = Rc::new(DiffuseLight::new_with_color(Color::with_scalar(4.0)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        diffuse_light.clone(),
    )));
    world.add(Rc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
        diffuse_light.clone(),
    )));

    let mut camera = Camera::zero();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;
    camera.background = Color::zero();
    camera.vertical_fov = 20.0;
    camera.center = Point3::new(26.0, 3.0, 6.0);
    camera.look_at = Point3::new(0.0, 2.0, 0.0);
    camera.up_direction = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;

    let ray_generate_time = std::time::Instant::now();
    camera.render(&world);
    eprintln!("End simple_light. {:.2?}", ray_generate_time.elapsed());
}
