use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use crate::model::ray::Ray;
use crate::model::vec3::Vec3;

pub(crate) fn prepare_render_image_file(path: &Path, width: i32, height: i32) -> File {
    let mut image = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(path) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't create {}: {}", path.display(), e)
    };
    match image.write(format!("P3\n{} {}\n255\n", width, height).as_ref()) {
        Ok(_) => {}
        Err(e) => panic!("Couldn't write to {}: {}", path.display(), e)
    }
    image
}

pub(crate) fn write_pixel(image: &mut File, path: &Path,  color: Vec3) {
    match image.write(format!("{} {} {}\n", color.x(), color.y(), color.z()).as_ref()) {
        Ok(_) => {}
        Err(e) => panic!("Couldn't write to {}: {}", path.display(), e)
    }
}
