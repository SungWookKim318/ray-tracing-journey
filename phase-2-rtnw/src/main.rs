#![allow(dead_code)]
use crate::scenes::{
    bouncing_spheres::bouncing_spheres, bouncing_spheres_with_bvh::bouncing_spheres_with_bvh,
    checkered_spheres::checkered_spheres,
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
}

fn main() {
    let mut input = String::new();
    eprintln!("Select Rendering Scenes.");
    eprintln!("1) bouncing_spheres");
    eprintln!("2) bouncing_spheres_with_bvh");
    eprintln!("3) checkered_spheres");
    eprintln!("*IMPORTANT* Current Default is 3.");
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
        1 => SceneTypes::BouncingSpheres,
        2 => SceneTypes::BvhOptimization,
        3 => SceneTypes::TwoCheckered,
        _ => SceneTypes::TwoCheckered,
    };

    match scene_type {
        SceneTypes::BouncingSpheres => bouncing_spheres(),
        SceneTypes::BvhOptimization => bouncing_spheres_with_bvh(),
        SceneTypes::TwoCheckered => checkered_spheres(),
    };
}
