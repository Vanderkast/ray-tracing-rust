use std::path::Path;
use crate::model::object::Hit;
use crate::model::ray::Ray;
use crate::model::sphere::Sphere;
use crate::model::vec3::Vec3;
use crate::render::helpers::{prepare_render_image_file, write_pixel};

pub fn white_to_blue_with_sphere_units() {
    let width = 200;
    let height = 100;
    let path = Path::new("renders/lerp_sphere.ppm");
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
    let sphere = Sphere::from(Vec3::from_i32(0,0, -1), 0.5);
    let t = sphere.hit(ray);
    if t > 0.0 {
        let n = (ray.point_at(t) - Vec3::from_i32(0, 0, -1)).to_unit();
        let rgb_scale = 0.5 * (n + 1.0);
        Vec3::from_i32(
            (255.99 * rgb_scale.x()) as i32,
            (255.99 * rgb_scale.y()) as i32,
            (255.99 * rgb_scale.z()) as i32
        )
    } else {
        let unit_direction = ray.direction().to_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        let pixel = (1.0 - t) * Vec3::ONE + t * Vec3::from(0.5, 0.7, 1.0);
        Vec3::from_i32(
            (255.99 * pixel.x()) as i32,
            (255.99 * pixel.y()) as i32,
            (255.99 * pixel.z()) as i32
        )
    }
}
