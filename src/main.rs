mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use color::write_color;
use hittable::{HitRecord, Hittable};
use ray::Ray;
use sphere::Sphere;
use std::io::Write;
use vec3::{Color, Vec3};

fn ray_color(ray: &Ray) -> Color {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let mut record = HitRecord::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0);
    let sphere = Sphere::new(center, 0.5);
    if sphere.hit(ray, 0.0, std::f64::INFINITY, &mut record) {
        let c = 0.5
            * Color::new(
                record.normal.x + 1.0,
                record.normal.y + 1.0,
                record.normal.z + 1.0,
            );
        return c;
    }

    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blue: Color = Color::new(0.5, 0.7, 1.0);

    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - t) * white + t * blue;
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
