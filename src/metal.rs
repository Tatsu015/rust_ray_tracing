use crate::{
    hittable::HitRecord,
    material::Material,
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
    fn scatter(
        &self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray_in.dir.reflect(record.normal).unit_vector();
        *scattered = Ray::new(record.p, reflected);
        *attenuation = self.albedo;
        return Vec3::dot(scattered.dir, record.normal) > 0.0;
    }
}
