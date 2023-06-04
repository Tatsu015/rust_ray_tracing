mod camera;
mod color;
mod dielectric;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod math;
mod metal;
mod ray;
mod sphere;
mod vec3;

extern crate rand;

use rand::Rng;

use crate::vec3::Point;
use camera::Camera;
use color::write_color;
use dielectric::Dielectric;
use hittable::Hittable;
use hittable_list::HittableList;
use lambertian::Lambertian;
use metal::Metal;
use ray::Ray;
use sphere::Sphere;
use std::io::Write;
use vec3::{Color, Vec3};

fn random_scene() -> HittableList {
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.45,
        Box::new(Dielectric::new(1.5)),
    )));

    return world;
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Color {
    if depth <= 0 {
        return Color::default();
    }
    let result = world.hit(ray, 0.0001, std::f64::INFINITY);
    if let Some(record) = result {
        let result = record.material.scatter(&ray, &record);
        if let Some(v) = result {
            return v.attenuation * ray_color(&v.ray, &world, depth - 1);
        }

        return Color::default();
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
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLE_PER_PIXCEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let lookfrom = Point::new(3.0, 3.0, 2.0);
    let lookat = Point::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        90.0,
        ASPECT_RATIO,
        2.0,
        (lookfrom - lookat).length(),
    );

    let world = random_scene();

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
                pixcel_sum_color += ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(pixcel_sum_color, SAMPLE_PER_PIXCEL);
        }
    }
    eprintln!("DONE");
}
