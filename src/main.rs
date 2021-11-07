use std::fs::OpenOptions;
use std::io::Write;
use std::ops;
use std::path::Path;
use crate::render::hello_world_gradient::hello_world_gradient;
use crate::render::white_to_blue_gradient::white_to_blue_gradient;
use crate::render::white_to_blue_with_sphere_units::white_to_blue_with_sphere_units;

mod model;
mod render;

fn main() {
    hello_world_gradient();
    white_to_blue_gradient();
    white_to_blue_with_sphere_units();
}
