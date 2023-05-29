use crate::{
    hittable::HitRecord,
    material::Material,
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
    fn scatter(
        &self,
        _: &Ray,
        record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new(record.p, Vec3::random_in_hemisphere(record.normal));
        *attenuation = self.albedo;

        return true;
    }
}
