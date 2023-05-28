use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        return HitRecord {
            p: Point::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
        };
    }
    pub fn new(p: Point, normal: Vec3, t: f64, front_face: bool) -> HitRecord {
        return HitRecord {
            p,
            normal,
            t,
            front_face,
        };
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
