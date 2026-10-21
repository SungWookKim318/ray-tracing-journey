#![allow(dead_code)]
use crate::scenes::{
    basic_quads::basic_quads, bouncing_spheres::bouncing_spheres,
    bouncing_spheres_with_bvh::bouncing_spheres_with_bvh, checkered_spheres::checkered_spheres,
    cornell_box::cornell_box, cornell_smoke::cornell_smoke, earth::earth_globe,
    final_scene::final_scene, perlin_spheres::perlin_spheres, simple_light::simple_light,
};
use core::panic;
use std::{
    eprintln,
    io::{self, Write},
};

mod color;
mod image_data;
mod objects;
mod ray;
mod scenes;
mod utils;
mod vec3;

enum SceneTypes {
    BouncingSpheres,
    BvhOptimization,
    TwoCheckered,
    EarthGlobe,
    PerlinSpheres,
    BasicQuads,
    SimpleLight,
    CornellBox,
    CornellSmoke,
    FinalSceneLow,
    FinalSceneHigh,
}

fn main() {
    let mut input = String::new();
    eprintln!("Select Rendering Scenes.");
    eprintln!("1) basic_quads");
    eprintln!("2) bouncing_spheres");
    eprintln!("3) bouncing_spheres_with_bvh");
    eprintln!("4) checkered_spheres");
    eprintln!("5) earth");
    eprintln!("6) perlin_spheres");
    eprintln!("7) simple_light");
    eprintln!("8) cornell_box");
    eprintln!("9) cornell_smoke");
    eprintln!("10) final_scene high quality");
    eprintln!("11) final_scene low quality");
    eprintln!("*IMPORTANT* Current Default is 1.");
    if io::stdout().flush().is_err() {
        panic!("std out stream is fail to flush.");
    }
    if io::stdin().read_line(&mut input).is_err() {
        eprintln!("using default number");
        input = String::from("");
    }

    let input_trim = match input.trim().parse() {
        Ok(number) => number,
        Err(error) => {
            eprintln!("fail to trimming reson {}", error);
            0
        }
    };

    let scene_type: SceneTypes = match input_trim {
        1 => SceneTypes::BasicQuads,
        2 => SceneTypes::BouncingSpheres,
        3 => SceneTypes::BvhOptimization,
        4 => SceneTypes::TwoCheckered,
        5 => SceneTypes::EarthGlobe,
        6 => SceneTypes::PerlinSpheres,
        7 => SceneTypes::SimpleLight,
        8 => SceneTypes::CornellBox,
        9 => SceneTypes::CornellSmoke,
        10 => SceneTypes::FinalSceneHigh,
        11 => SceneTypes::FinalSceneLow,

        _ => SceneTypes::BasicQuads,
    };

    match scene_type {
        SceneTypes::BouncingSpheres => bouncing_spheres(),
        SceneTypes::BvhOptimization => bouncing_spheres_with_bvh(),
        SceneTypes::TwoCheckered => checkered_spheres(),
        SceneTypes::EarthGlobe => earth_globe(),
        SceneTypes::PerlinSpheres => perlin_spheres(),
        SceneTypes::BasicQuads => basic_quads(),
        SceneTypes::SimpleLight => simple_light(),
        SceneTypes::CornellBox => cornell_box(),
        SceneTypes::CornellSmoke => cornell_smoke(),
        SceneTypes::FinalSceneHigh => final_scene(800, 1000, 40),
        SceneTypes::FinalSceneLow => final_scene(400, 250, 4),
    };
}
