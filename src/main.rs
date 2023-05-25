mod color;
mod vec3;

use color::write_color;
use vec3::Vec3;

use std::io::Write;

fn main() {
    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for i in (0..HEIGHT).rev() {
        eprint!("\rremain {}", i);
        std::io::stdout().flush().unwrap();
        for j in 0..WIDTH {
            let r: f64 = f64::from(j) / f64::from(WIDTH - 1);
            let g: f64 = f64::from(i) / f64::from(HEIGHT - 1);
            let b: f64 = 0.25;

            let c = Vec3 { x: r, y: g, z: b };
            write_color(c);
        }
    }
    eprintln!("DONE");
}
