use std::{eprint, panic, rc::Rc};

use crate::{
    objects::{
        camera::Camera,
        materials::{lambertian::Lambertian, material::Material},
        sphere::Sphere,
        textures::image_texture::{ImageTexture, PixelType},
    },
    vec3::{Point3, Vec3},
};

pub(crate) fn earth_globe() {
    eprint!("start earth_globe");
    let image_path = "assets/earthmap.jpg";
    let texture = match ImageTexture::load_file(image_path.to_string(), PixelType::RGB888) {
        Some(texture) => Rc::new(texture),
        None => {
            panic!("fail to load image");
        }
    };
    let surface: Rc<dyn Material> = Rc::new(Lambertian::new_with_texture(texture));
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, surface);

    let mut camera = Camera::zero();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.sample_per_pixel = 100;
    camera.max_depth = 50;

    camera.vertical_fov = 20.0;
    camera.center = Point3::new(0.0, 0.0, 12.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.up_direction = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    let ray_generate_time = std::time::Instant::now();
    camera.render(&globe);
    eprintln!("End earth_globe. {:.2?}", ray_generate_time.elapsed());
}
