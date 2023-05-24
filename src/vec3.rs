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
}
