use std::fs::OpenOptions;
use std::io::Write;
use std::ops;
use std::path::Path;

mod model;
mod render;

fn main() {
    render::tracer::gradient();
    render::tracer::white_to_blue();
}
