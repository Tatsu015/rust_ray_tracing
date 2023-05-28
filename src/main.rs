mod camera;
mod color;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod sphere;
mod vec3;

extern crate rand;

use rand::Rng;

use camera::Camera;
use color::write_color;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::io::Write;
use vec3::{Color, Vec3};

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let mut record = HitRecord::default();

    if world.hit(ray, 0.0, std::f64::INFINITY, &mut record) {
        let c = 0.5 * (Color::new(1.0, 1.0, 1.0) + record.normal);
        return c;
    }

    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blue: Color = Color::new(0.5, 0.7, 1.0);
    let unit_dir = ray.dir.unit_vector();
    let t = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - t) * white + t * blue;
}

fn main() {
    let mut rng = rand::thread_rng();

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 384;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLE_PER_PIXCEL: u32 = 100;

    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new(ASPECT_RATIO);

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for i in (0..HEIGHT).rev() {
        eprint!("\rremain {:3}", i);
        std::io::stdout().flush().unwrap();
        for j in 0..WIDTH {
            let mut pixcel_sum_color = Color::default();
            for _ in 0..SAMPLE_PER_PIXCEL {
                let u = (f64::from(j) + rng.gen::<f64>()) / f64::from(WIDTH - 1);
                let v = (f64::from(i) + rng.gen::<f64>()) / f64::from(HEIGHT - 1);
                let ray = camera.get_ray(u, v);
                pixcel_sum_color += ray_color(&ray, &world);
            }
            write_color(pixcel_sum_color, SAMPLE_PER_PIXCEL);
        }
    }
    eprintln!("DONE");
}
