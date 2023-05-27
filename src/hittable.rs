use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
}

impl HitRecord {
    pub fn new(p: Point, normal: Vec3, t: f64) -> HitRecord {
        return HitRecord { p, normal, t };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
