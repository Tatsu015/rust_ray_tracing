use std::ops;

extern crate rand;
use rand::Rng;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Color = Vec3;
pub type Point = Vec3;

impl Vec3 {
    pub fn default() -> Vec3 {
        return Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        return Vec3 { x, y, z };
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let mut rng = rand::thread_rng();
        return Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        );
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_double() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0..2.0) * std::f64::consts::PI;
        let z: f64 = rng.gen_range(-1.0..1.0);
        let r: f64 = (1.0 - z * z).sqrt();
        return Vec3::new(r * f64::cos(a), r * f64::sin(a), z);
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
        let unit_sphere = Vec3::random_in_unit_sphere();
        if Vec3::dot(unit_sphere, normal) > 0.0 {
            return unit_sphere;
        } else {
            return -1.0 * unit_sphere;
        }
    }

    pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
        return lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z;
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        return Vec3::new(
            lhs.y * rhs.z - rhs.y * lhs.z,
            lhs.z * rhs.x - rhs.z * lhs.x,
            lhs.x * rhs.y - rhs.x * lhs.y,
        );
    }

    pub fn unit_vector(&self) -> Vec3 {
        return Vec3::new(self.x, self.y, self.z) / self.length();
    }

    pub fn length_double(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    pub fn length(&self) -> f64 {
        return self.length_double().sqrt();
    }

    pub fn clamp(&self, min: f64, max: f64) -> Vec3 {
        return Vec3::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        );
    }

    pub fn sqrt(&self) -> Vec3 {
        return Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt());
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        let length = 2.0 * Vec3::dot(*self, normal);
        return *self - length * normal;
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        return Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        return Vec3::new(self.x / rhs, self.y / rhs, self.z / rhs);
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        return Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self);
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.000001;

    #[test]
    fn test_length() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);

        let v2 = Vec3::new(-1.0, -2.0, -3.0);

        assert!((v1.length() * v1.length() - 14.0).abs() < EPSILON);
        assert!((v2.length() * v2.length() - 14.0).abs() < EPSILON);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert!(Vec3::dot(v1, v2) - 20.0 <= EPSILON);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        let ans = Vec3::cross(v1, v2);

        assert!(ans.x <= EPSILON);
        assert!(ans.y <= EPSILON);
        assert!(ans.z - 1.0 <= EPSILON);
    }

    #[test]
    fn test_unit_vector() {
        let v1 = Vec3::new(2.0, 0.0, 0.0);

        let ans = v1.unit_vector();

        assert!(ans.x - 1.0 <= EPSILON);
        assert!(ans.y <= EPSILON);
        assert!(ans.z <= EPSILON);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.1, 2.2, 3.3);
        let v2 = Vec3::new(4.4, 5.5, 6.6);

        let ans = v1 + v2;
        assert!((ans.x - 5.5).abs() <= EPSILON);
        assert!((ans.y - 7.7).abs() <= EPSILON);
        assert!((ans.z - 9.9).abs() <= EPSILON);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(1.1, 2.2, 6.6);
        let v2 = Vec3::new(4.4, 2.2, 2.2);

        let ans1 = v1 - v2;
        assert!((ans1.x + 3.3).abs() <= EPSILON);
        assert!((ans1.y).abs() <= EPSILON);
        assert!((ans1.z - 4.4).abs() <= EPSILON);

        let ans2 = v2 - v1;
        assert!((ans2.x - 3.3).abs() <= EPSILON);
        assert!((ans2.y).abs() <= EPSILON);
        assert!((ans2.z + 4.4).abs() <= EPSILON);
    }

    #[test]
    fn test_mul() {
        let v1 = Vec3::new(1.1, 2.2, 3.3);

        let ans1 = v1 * 2.0;
        assert!((ans1.x - 2.2).abs() <= EPSILON);
        assert!((ans1.y - 4.4).abs() <= EPSILON);
        assert!((ans1.z - 6.6).abs() <= EPSILON);

        let ans2 = 2.0 * v1;
        assert!((ans2.x - 2.2).abs() <= EPSILON);
        assert!((ans2.y - 4.4).abs() <= EPSILON);
        assert!((ans2.z - 6.6).abs() <= EPSILON);
    }

    #[test]
    fn test_div() {
        let v1 = Vec3::new(1.1, 2.2, 3.3);

        let ans1 = v1 / 2.0;
        assert!((ans1.x - 0.55).abs() <= EPSILON);
        assert!((ans1.y - 1.1).abs() <= EPSILON);
        assert!((ans1.z - 1.65).abs() <= EPSILON);
    }

    #[test]
    fn test_add_assign() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let mut ans = Vec3::new(4.0, 5.0, 6.0);

        ans += v1;
        assert!((ans.x - 5.0) <= EPSILON);
        assert!((ans.y - 7.0) <= EPSILON);
        assert!((ans.z - 9.0) <= EPSILON);
    }

    #[test]
    fn test_sub_assign() {
        let v1 = Vec3::new(1.1, 2.2, 6.6);
        let mut ans = Vec3::new(4.4, 2.2, 2.2);

        ans -= v1;

        assert!((ans.x - 3.3) <= EPSILON);
        assert!((ans.y) <= EPSILON);
        assert!((ans.z + 4.4) <= EPSILON);
    }

    #[test]
    fn test_mul_assign() {
        let mut ans = Vec3::new(1.1, 2.2, 3.3);

        ans *= 2.0;

        assert!((ans.x - 2.2) <= EPSILON);
        assert!((ans.y - 4.4) <= EPSILON);
        assert!((ans.z - 6.6) <= EPSILON);
    }

    #[test]
    fn test_div_assign() {
        let mut ans = Vec3::new(1.1, 2.2, 3.3);

        ans /= 2.0;

        assert!((ans.x - 0.55) <= EPSILON);
        assert!((ans.y - 1.1) <= EPSILON);
        assert!((ans.z - 1.65) <= EPSILON);
    }

    #[test]
    fn test_div_reflect() {
        let v1 = Vec3::new(2.0, 3.0, 4.0);
        let n = Vec3::new(1.0, 1.0, 1.0);
        let ans = v1.reflect(n);

        assert!((ans.x + 2.0) <= EPSILON);
        assert!((ans.y + 3.0) <= EPSILON);
        assert!((ans.z + 4.0) <= EPSILON);
    }
}
