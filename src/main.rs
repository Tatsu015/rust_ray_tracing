mod color;
mod ray;
mod vec3;

use color::write_color;
use ray::Ray;
use std::io::Write;
use vec3::{Color, Point, Vec3};

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blue: Color = Color::new(0.5, 0.7, 1.0);

    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - t) * white + t * blue;
}

fn hit_sphere(center: Point, radius: f64, r: &Ray) -> bool {
    let oc = r.org - center;
    let a = Vec3::dot(r.dir, r.dir);
    let b = Vec3::dot(r.dir, oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let d = b * b - a * c;

    if d > 0.0 {
        return true;
    }
    return false;
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: i32 = 384;
    const HEIGHT: i32 = ((WIDTH as f64) / ASPECT_RATIO) as i32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * ASPECT_RATIO;
    let focal_length = 1.0;

    let org = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let focal_center = Vec3::new(0.0, 0.0, focal_length);
    let lower_left_corner = org - horizontal / 2.0 - vertical / 2.0 - focal_center;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for i in (0..HEIGHT).rev() {
        eprint!("\rremain {}", i);
        std::io::stdout().flush().unwrap();
        for j in 0..WIDTH {
            let u = f64::from(j) / f64::from(WIDTH - 1);
            let v = f64::from(i) / f64::from(HEIGHT - 1);
            let ray = Ray::new(org, lower_left_corner + u * horizontal + v * vertical - org);
            let c = ray_color(&ray);
            write_color(c);
        }
    }
    eprintln!("DONE");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hit_sphere() {
        let org = Vec3::new(0.0, 0.0, 0.0);
        let sphere_ray_hit = Vec3::new(0.0, 0.0, 1.0);
        let sphere_ray_hit_limit = Vec3::new(0.0, 0.0, 1.4);
        let sphere_ray_not_hit = Vec3::new(0.0, 0.0, 1.5);
        let radius = 1.0;

        let r_xz = Ray::new(org, Vec3::new(1.0, 0.0, 1.0));

        let sut = hit_sphere(sphere_ray_hit, radius, &r_xz);
        assert_eq!(sut, true);
        let sut = hit_sphere(sphere_ray_hit_limit, radius, &r_xz);
        assert_eq!(sut, true);
        let sut = hit_sphere(sphere_ray_not_hit, radius, &r_xz);
        assert_eq!(sut, false);

        let r_yz = Ray::new(org, Vec3::new(0.0, 1.0, 1.0));

        let sut = hit_sphere(sphere_ray_hit, radius, &r_yz);
        assert_eq!(sut, true);
        let sut = hit_sphere(sphere_ray_hit_limit, radius, &r_yz);
        assert_eq!(sut, true);
        let sut = hit_sphere(sphere_ray_not_hit, radius, &r_yz);
        assert_eq!(sut, false);
    }
}
