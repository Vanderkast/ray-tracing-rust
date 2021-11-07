use crate::model::vec3::Vec3;

pub struct RgbScale(Vec3);

pub struct Color {
    r: i32,
    g: i32,
    b: i32
}

impl Color {
    pub fn from(r: i32, g: i32, b: i32) -> Color {
        Color {
            r,
            g,
            b
        }
    }

    pub fn from_scale(scales: RgbScale) -> Color {
        Color {
            r: (255.99 * scales.0.x()) as i32,
            g: (255.99 * scales.0.y()) as i32,
            b: (255.99 * scales.0.z()) as i32
        }
    }
}
