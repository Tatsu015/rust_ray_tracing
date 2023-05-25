use std::ops;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn length_double(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }

    fn length(&self) -> f64 {
        return self.length_double().sqrt();
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        return Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        return Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        return Vec3 {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.000001;

    #[test]
    fn test_add() {
        let v1 = Vec3 {
            x: 1.1,
            y: 2.2,
            z: 3.3,
        };
        let v2 = Vec3 {
            x: 4.4,
            y: 5.5,
            z: 6.6,
        };

        let ans = v1 + v2;
        assert!((ans.x - 5.5).abs() <= EPSILON);
        assert!((ans.y - 7.7).abs() <= EPSILON);
        assert!((ans.z - 9.9).abs() <= EPSILON);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3 {
            x: 1.1,
            y: 2.2,
            z: 6.6,
        };
        let v2 = Vec3 {
            x: 4.4,
            y: 2.2,
            z: 2.2,
        };

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
        let v1 = Vec3 {
            x: 1.1,
            y: 2.2,
            z: 3.3,
        };

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
    fn test_add_assign() {
        let v1 = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let mut ans = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };

        ans += v1;

        assert!((ans.x - 5.0) <= EPSILON);
        assert!((ans.y - 7.0) <= EPSILON);
        assert!((ans.z - 9.0) <= EPSILON);
    }

    #[test]
    fn test_sub_assign() {
        let v1 = Vec3 {
            x: 1.1,
            y: 2.2,
            z: 6.6,
        };
        let mut ans = Vec3 {
            x: 4.4,
            y: 2.2,
            z: 2.2,
        };

        ans -= v1;

        assert!((ans.x - 3.3) <= EPSILON);
        assert!((ans.y) <= EPSILON);
        assert!((ans.z + 4.4) <= EPSILON);
    }
}
