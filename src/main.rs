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
use std::{fs::File, time::Instant};

use crate::{color::to_string, vec3::Point};
use camera::Camera;
use dielectric::Dielectric;
use hittable::Hittable;
use hittable_list::HittableList;
use lambertian::Lambertian;
use metal::Metal;
use ray::Ray;
use sphere::Sphere;
use std::io::Write;
use vec3::{Color, Vec3};

const RANGE: i32 = 11;

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    // ground
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5))),
    )));

    for a in -RANGE..RANGE {
        for b in -RANGE..RANGE {
            let mut rng = rand::thread_rng();
            let choose_mat = rng.gen_range(0.0..1.0);
            let center = Point::new(
                (a as f64) + 0.9 * rng.gen_range(0.0..1.0),
                0.2,
                (b as f64) * 0.9 * rng.gen_range(0.0..1.0),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random(0.5, 1.0);
                    let fizz = rng.gen_range(0.0..0.5);
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Metal::new(albedo, fizz)),
                    )));
                } else {
                    // glass
                    world.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Box::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
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
    let start = Instant::now();
    let mut rng = rand::thread_rng();

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const WIDTH: u32 = 200;
    const HEIGHT: u32 = ((WIDTH as f64) / ASPECT_RATIO) as u32;
    const SAMPLE_PER_PIXCEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let world = random_scene();

    let mut buf: String;

    buf = format!("P3\n{} {}\n255\n", WIDTH, HEIGHT);

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
            let cs = to_string(pixcel_sum_color, SAMPLE_PER_PIXCEL);
            buf.push_str(&cs);
        }
    }

    let mut file = File::create("out.ppm").expect("Failed to create file");

    file.write_all(buf.as_bytes())
        .expect("Failed to write data to file");

    file.flush().expect("Failed to flush data to file");

    let end = Instant::now();
    eprintln!("\rElapsed time: {:?}", end - start);
}
