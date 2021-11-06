use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::render::ray::Ray;
use crate::render::vec3::Vec3;

fn prepare_render_image_file(path: &Path, width: i32, height: i32) -> File {
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

fn write_pixel(image: &mut File, path: &Path,  color: Vec3) {
    match image.write(format!("{} {} {}\n", color.x(), color.y(), color.z()).as_ref()) {
        Ok(_) => {}
        Err(e) => panic!("Couldn't write to {}: {}", path.display(), e)
    }
}

pub fn white_to_blue() {
    let width = 200;
    let height = 100;
    let path = Path::new("renders/lerp.ppm");
    let mut image = prepare_render_image_file(path, width, height);

    let lower_left_corner = Vec3::from_i32(-2, -1, -1);
    let horizontal = Vec3::from_i32(4, 0, 0);
    let vertical = Vec3::from_i32(0, 2, 0);
    let camera = Vec3::zero();

    for y in (0..height).rev() {
        for x in 0..width {
            let u = (x as f32) / width as f32;
            let v = (y as f32) / height as f32;
            let direction = lower_left_corner + u*horizontal + v * vertical;
            let ray = Ray::from(camera, direction);
            write_pixel(&mut image, path, pixel_color(ray));
        }
    }
}

pub fn gradient() {
    let width = 200;
    let height = 100;
    let path = Path::new("renders/image.ppm");
    let mut image = prepare_render_image_file(path, width, height);

    for y in (0..height).rev() {
        for x in 0..width {
            let r = (x as f32) / (width as f32);
            let g = (y as f32) / (height as f32);
            let b: f32 = 0.2;
            let pixel = Vec3::from_i32(
                (COLOR * r) as i32,
                (COLOR * g) as i32,
                (COLOR * b) as i32
            );

            write_pixel(&mut image, path, pixel);
        }
    }
}

static COLOR: f32 = 255.99;

pub fn pixel_color(ray: Ray) -> Vec3 {
    let pixel = ray.pixel();
    Vec3::from_i32(
        (COLOR * pixel.x()) as i32,
        (COLOR * pixel.y()) as i32,
        (COLOR * pixel.z()) as i32
    )
}
