use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct HitRecord<'a> {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        p: Point,
        normal: Vec3,
        t: f64,
        front_face: bool,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
        return HitRecord {
            p,
            normal,
            t,
            front_face,
            material,
        };
    }

    pub fn is_front_face(ray: &Ray, outward_normal: Vec3) -> bool {
        return Vec3::dot(ray.dir, outward_normal) < 0.0;
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
