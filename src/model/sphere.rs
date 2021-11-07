use std::collections::VecDeque;
use crate::model::object::Hit;
use crate::model::ray::Ray;
use crate::model::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn from(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: Ray) -> f32 {
        let origin_center = ray.origin() - self.center;
        let a = Vec3::dot_product(ray.direction(), ray.direction());
        let b = 2.0 * Vec3::dot_product(origin_center, ray.direction());
        let c = Vec3::dot_product(origin_center, origin_center) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            -1.0
        } else {
            // ???-discriminant is used because we everything in front of camera has negative z???
            // todo why we use -sqrt(disc) in sphere hit registration function
            (-b - f32::sqrt(discriminant)) / 2.0 * a
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hits_true() {
        // given
        let ray = Ray::from(
            Vec3::from_i32(0, 0, 0),
            Vec3::from_i32(1, 1, 0),
        );
        let sphere = Sphere::from(Vec3::from_i32(1, 1, 1), 1.0);
        // when
        let actual = sphere.hit(ray);
        // then
        assert!(actual > 0.0)
    }
}
