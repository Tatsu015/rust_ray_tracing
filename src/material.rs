use crate::{hittable::HitRecord, ray::Ray, vec3::Color};

pub struct Scattered {
    pub ray: Ray,
    pub attenuation: Color,
}

impl Scattered {
    pub fn new(ray: Ray, attenuation: Color) -> Scattered {
        return Scattered { ray, attenuation };
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<Scattered>;
}
