use std::fs::OpenOptions;
use std::io::Write;
use std::ops;
use std::path::Path;

mod model;
use model::vec3::Vec3;

fn main() {
    // write_gradient();
    let one = Vec3::one();
    println!("For {:?} len is {}, and squared len is {}", one, one.len(), one.square_len())
}

fn write_gradient() {
    let width = 200;
    let height = 100;
    let path = Path::new("image.ppm");

    // let mut image = match fs::File::create("image.ppm") {
    let mut image = match OpenOptions::new()
        .write(true)
        .create(true)
        .open(path) {
        Ok(f) => f,
        Err(e) => panic!("Couldn't create {}: {}", path.display(), e)
    };

    // writing image parameters
    match image.write(format!("P3\n{} {}\n255\n", width, height).as_ref()) {
        Ok(_) => {}
        Err(e) => panic!("Couldn't write to {}: {}", path.display(), e)
    };

    for y in 0..height {
        for x in 0..width {
            let r = (x as f32) / (width as f32);
            let g = (y as f32) / (height as f32);
            let b: f32 = 0.2;
            let p_r = (255.99 * r) as i32;
            let p_g = (255.99 * g) as i32;
            let p_b = (255.99 * b) as i32;

            match image.write(format!("{} {} {}\n", p_r, p_g, p_b).as_ref()) {
                Ok(_) => {}
                Err(e) => panic!("Couldn't write to {}: {}", path.display(), e)
            }
        }
    }
}
