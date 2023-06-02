use crate::{
    hittable::HitRecord,
    material::{Material, Scattered},
    ray::Ray,
    vec3::{Color, Vec3},
};

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        return Dielectric { refractive_index };
    }

    fn refract(&self, unit_ray_in_dir: Vec3, normal: Vec3, eta_in_over_out: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(-1.0 * unit_ray_in_dir, normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if eta_in_over_out * sin_theta > 1.0 {}
        let out_parallel: Vec3 = eta_in_over_out * (unit_ray_in_dir + cos_theta * normal);
        let out_perpendicular = -(1.0 - out_parallel.length_double()).sqrt() * normal;

        return out_parallel + out_perpendicular;
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<Scattered> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let eta_in_over_out = if record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_ray_in_dir = ray_in.dir.unit_vector();
        let cos_theta = f64::min(Vec3::dot(-1.0 * unit_ray_in_dir, record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if eta_in_over_out * sin_theta > 1.0 {
            let reflected = ray_in.dir.reflect(record.normal).unit_vector();
            let scattered = Scattered::new(Ray::new(record.p, reflected), attenuation);
            return Some(scattered);
        }

        let refract = self.refract(unit_ray_in_dir, record.normal, eta_in_over_out);
        let scattered = Scattered::new(Ray::new(record.p, refract), attenuation);
        return Some(scattered);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.000001;

    #[test]
    fn test_refract() {
        let d = Dielectric::new(1.0);
        let unit_ray_in_dir = Vec3::new(1.0, 0.0, 0.0);
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let sut = d.refract(unit_ray_in_dir, normal, 1.0);
        assert!(sut.x - 1.0 <= EPSILON);
        assert!(sut.y - 0.0 <= EPSILON);
        assert!(sut.z - 0.0 <= EPSILON);
    }
}
