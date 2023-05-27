use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        return Sphere { center, radius };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.org - self.center;
        let a = Vec3::dot(ray.dir, ray.dir);
        let b = Vec3::dot(ray.dir, oc);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let d = b * b - a * c;

        if d > 0.0 {
            let t = (-b - d.sqrt()) / a;
            if t_min < t && t < t_max {
                record.t = t;
                record.p = ray.at(t);
                let outward_normal = (record.p - self.center).unit_vector();
                record.set_face_normal(ray, outward_normal);
                return true;
            }

            let t = (-b + d.sqrt()) / a;
            if t_min < t && t < t_max {
                record.t = t;
                record.p = ray.at(t);
                let outward_normal = (record.p - self.center).unit_vector();
                record.set_face_normal(ray, outward_normal);
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hit() {
        let org = Vec3::new(0.0, 0.0, 0.0);
        let sphere_ray_hit = Vec3::new(0.0, 0.0, 1.4);
        let sphere_ray_not_hit = Vec3::new(0.0, 0.0, 1.5);
        let radius = 1.0;
        let mut record = HitRecord::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            0.0,
            false,
        );

        // for xz plane
        let r_xz = Ray::new(org, Vec3::new(1.0, 0.0, 1.0));
        let sp = Sphere::new(sphere_ray_hit, radius);
        let sut = sp.hit(&r_xz, 0.0, std::f64::INFINITY, &mut record);
        assert_eq!(sut, true);

        let sp = Sphere::new(sphere_ray_not_hit, radius);
        let sut = sp.hit(&r_xz, 0.0, std::f64::INFINITY, &mut record);
        assert_eq!(sut, false);

        // for xz plane
        let r_yz = Ray::new(org, Vec3::new(0.0, 1.0, 1.0));
        let sp = Sphere::new(sphere_ray_hit, radius);
        let sut = sp.hit(&r_yz, 0.0, std::f64::INFINITY, &mut record);
        assert_eq!(sut, true);

        let sp = Sphere::new(sphere_ray_not_hit, radius);
        let sut = sp.hit(&r_yz, 0.0, std::f64::INFINITY, &mut record);
        assert_eq!(sut, false);
    }
}
