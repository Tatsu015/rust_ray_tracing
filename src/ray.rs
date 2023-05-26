use crate::vec3::{Point, Vec3};

pub struct Ray {
    pub org: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Point {
        return self.org + self.dir * t;
    }
    pub fn new(org: Vec3, dir: Vec3) -> Ray {
        return Ray { org, dir };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_at() {
        let o = Vec3::new(1.0, 1.0, 1.0);
        let d = Vec3::new(1.0, 2.0, 3.0);
        let r = Ray::new(o, d);

        let sut = r.at(2.0);

        assert_eq!(sut.x, 3.0);
        assert_eq!(sut.y, 5.0);
        assert_eq!(sut.z, 7.0);
    }
}
