use crate::{
    math::degrees_to_radians,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta * 0.5).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).unit_vector();
        let u = Vec3::cross(vup, w).unit_vector();
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        return Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        };
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        );
    }
}
