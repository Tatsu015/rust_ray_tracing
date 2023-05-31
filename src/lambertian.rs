use crate::{
    hittable::HitRecord,
    material::{Material, Scattered},
    ray::Ray,
    vec3::{Color, Vec3},
};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        return Lambertian { albedo };
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<Scattered> {
        let ray = Ray::new(record.p, Vec3::random_in_hemisphere(record.normal));
        let attenuation = self.albedo;

        let scattered = Scattered::new(ray, attenuation);
        return Some(scattered);
    }
}
