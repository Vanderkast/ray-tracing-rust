use std::iter::FromIterator;
use std::ops;
use std::path::Iter;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl FromIterator<f32> for Vec3 {
    fn from_iter<T: IntoIterator<Item=f32>>(iter: T) -> Self {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for (i, v)
        in iter.into_iter().enumerate().take(3) {
            match i {
                0 => x = v,
                1 => y = v,
                2 => z = v,
                _ => {}
            }
        }
        Vec3 {
            x,
            y,
            z,
        }
    }
}

impl Vec3 {
    pub fn from(x: f32, y: f32, z: f32) -> Self {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn from_i32(x: i32, y: i32, z: i32) -> Self {
        Vec3 {
            x: x as f32,
            y: y as f32,
            z: z as f32,
        }
    }

    pub fn zero() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn one() -> Self {
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn square_len(&self) -> f32 {
        self.x * self.x
            + self.y * self.y
            + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        f32::sqrt(self.square_len())
    }

    pub fn to_unit(&self) -> Vec3 {
       ( 1.0 / self.len()) * (*self)
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.x, self.y, self.z)
    }
}

/// Addition operator + for vec3 over f32
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Subtract operator - for vec3 over f32
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Unary negation operator for vec3 over f32
impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Left hand side scalar product operator * for vec3 over f32
/// if v: Vec3, and k: f32
/// then k * v = s: Vec3
/// where s.x = k * v.x; s.y = k * v.y; x.z = k * v.z
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

/// Left hand side scalar division operator / for vec3 over f32
/// Equals to 1.0/k*v, for 1.0: f32; k: f32; v: vec3
impl ops::Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::*;

    #[test]
    fn eq() {
        // given
        let a = Vec3::from_i32(1, 1, 1);
        let b = Vec3::from_i32(1, 1, 1);
        // when
        let actual = a == b;
        // then
        assert!(actual);
    }

    #[test]
    fn not_eq() {
        // given
        let a = Vec3::from_i32(1, 1, 1);
        let b = Vec3::from_i32(1, 2, 1);
        // when
        let actual = a != b;
        // then
        assert!(actual);
    }

    #[test]
    fn addition() {
        // given
        let a = Vec3::from_i32(1, 1, 1);
        let b = Vec3::from_i32(2, 2, 2);
        // when
        let actual = a + b;
        // then
        let expected = Vec3::from_i32(3, 3, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn subtraction() {
        // given
        let a = Vec3::from_i32(1, 1, 1);
        let b = Vec3::from_i32(2, 2, 2);
        // when
        let actual = a - b;
        // then
        let expected = Vec3::from_i32(-1, -1, -1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn scalar_product() {
        // given
        let a = Vec3::from_i32(1, -2, 3);
        let k = 7.0;
        // when
        let actual = k * a;
        // then
        let expected = Vec3::from_i32(7, -14, 21);
        assert_eq!(expected, actual);
    }

    #[test]
    fn zero() {
        let zero = Vec3::zero();

        // addition
        let a = Vec3::from_i32(1, -2, 3);
        let sum = zero + a;
        assert_eq!(a, sum);

        // subtraction
        let a = Vec3::from_i32(3, -2, 1);
        let sub = zero - a;
        let expected = -1.0 * a;
        assert_eq!(expected, sub);

        // scale
        let k = 4.0;
        let expected = Vec3::zero();
        let scaled = k * zero;
        assert_eq!(expected, scaled);
    }

    #[test]
    fn one() {
        let one = Vec3::one();

        //
    }

    #[test]
    fn from_iterator() {
        let expected = Vec3::from_i32(1, 2, 3);
        let a: Vec3 = vec![1.0, 2.0, 3.0, 4.0].into_iter().collect();
        assert_eq!(expected, a);
    }

    #[test]
    fn to_string() {
        let a = Vec3::from(12.0, -3.99, 7.778);
        let actual = a.to_string();
        assert_eq!("12 -3.99 7.778", actual)
    }

    #[test]
    fn to_unit() {
        // given
        let a = Vec3::from_i32(3, 2, 1);
        let a_len = a.len();
        let expected = Vec3::from(
            a.x / a_len,
            a.y / a_len,
            a.z / a_len
        );
        //when
        let actual = a.to_unit();
        // then
        assert_eq!(expected, actual);
    }
}
