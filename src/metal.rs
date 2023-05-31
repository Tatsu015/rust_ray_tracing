use crate::{
    hittable::HitRecord,
    material::{Material, Scattered},
    ray::Ray,
    vec3::{Color, Vec3},
};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Metal {
        return Metal { albedo };
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<Scattered> {
        let reflected = ray_in.dir.reflect(record.normal).unit_vector();
        let ray = Ray::new(record.p, reflected);
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
