use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<dyn Material>) -> Sphere {
        return Sphere {
            center,
            radius,
            material,
        };
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.org - self.center;
        let a = Vec3::dot(ray.dir, ray.dir);
        let b = Vec3::dot(ray.dir, oc);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let d = b * b - a * c;

        if d > 0.0 {
            let t = (-b - d.sqrt()) / a;
            if t_min < t && t < t_max {
                let p = ray.at(t);
                let outward_normal = (p - self.center).unit_vector();
                let front_face = HitRecord::is_front_face(ray, outward_normal);
                let record = HitRecord::new(p, outward_normal, t, front_face, &*self.material);
                return Some(record);
            }

            let t = (-b + d.sqrt()) / a;
            if t_min < t && t < t_max {
                let p = ray.at(t);
                let outward_normal = (p - self.center).unit_vector();
                let front_face = HitRecord::is_front_face(ray, outward_normal);
                let record = HitRecord::new(p, outward_normal, t, front_face, &*self.material);
                return Some(record);
            }
        }

        return None;
    }
}
