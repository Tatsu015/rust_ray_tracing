use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

pub trait Material {
    fn scatter(ray_in: &Ray, record: &HitRecord, attenuation: &Color, scattered: &mut Ray) -> bool;
}
