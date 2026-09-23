#![allow(dead_code)]
use crate::scenes::{
    basic_quads::basic_quads, bouncing_spheres::bouncing_spheres,
    bouncing_spheres_with_bvh::bouncing_spheres_with_bvh, checkered_spheres::checkered_spheres,
    earth::earth_globe, perlin_spheres::perlin_spheres,
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

        _ => SceneTypes::BasicQuads,
    };

    match scene_type {
        SceneTypes::BouncingSpheres => bouncing_spheres(),
        SceneTypes::BvhOptimization => bouncing_spheres_with_bvh(),
        SceneTypes::TwoCheckered => checkered_spheres(),
        SceneTypes::EarthGlobe => earth_globe(),
        SceneTypes::PerlinSpheres => perlin_spheres(),
        SceneTypes::BasicQuads => basic_quads(),
    };
}
