use crate::{
    hittable::HitRecord,
    material::{Material, Scattered},
    ray::Ray,
    vec3::{Color, Vec3},
};

pub struct Metal {
    albedo: Color,
    fizz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fizz: f64) -> Metal {
        let rounded_fizz = if fizz < 1.0 { fizz } else { 1.0 };
        return Metal {
            albedo,
            fizz: rounded_fizz,
        };
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<Scattered> {
        let reflected = ray_in.dir.reflect(record.normal).unit_vector();
        let ray = Ray::new(
            record.p,
            reflected + self.fizz * Vec3::random_in_unit_sphere(),
        );
        let attenuation = self.albedo;

        let is_reflected = Vec3::dot(ray.dir, record.normal) > 0.0;
        if is_reflected {
            let scattered = Scattered::new(ray, attenuation);
            return Some(scattered);
        } else {
            return None;
        }
    }
}
