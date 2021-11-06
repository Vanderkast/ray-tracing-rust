use crate::render::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn from(origin: Vec3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn pixel(&self) -> Vec3 {
        let unit_direction = self.direction;
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::from(0.5, 0.7, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_at() {
        let ray = Ray {
            origin: Vec3::zero(),
            direction: Vec3::from_i32(2, 2, 2)
        };
        let t = 0.5;
        assert_eq!(Vec3::one(), ray.point_at(t));
    }
}
