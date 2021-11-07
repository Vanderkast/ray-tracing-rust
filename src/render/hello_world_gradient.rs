use std::path::Path;
use crate::model::ray::Ray;
use crate::model::vec3::Vec3;
use crate::render::helpers::*;

pub fn hello_world_gradient() {
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
                (255.99 * r) as i32,
                (255.99 * g) as i32,
                (255.99 * b) as i32
            );

            write_pixel(&mut image, path, pixel);
        }
    }
}
