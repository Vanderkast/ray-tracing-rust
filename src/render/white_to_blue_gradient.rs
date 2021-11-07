use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::model::ray::Ray;
use crate::model::vec3::Vec3;
use crate::render::helpers::*;

pub fn white_to_blue_gradient() {
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

fn pixel_color(ray: Ray) -> Vec3 {
    let unit_direction = ray.direction();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let pixel = (1.0 - t) * Vec3::ONE + t * Vec3::from(0.5, 0.7, 1.0);
    Vec3::from_i32(
        (255.99 * pixel.x()) as i32,
        (255.99 * pixel.y()) as i32,
        (255.99 * pixel.z()) as i32
    )
}
