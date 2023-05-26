mod color;
mod vec3;

use color::write_color;
use vec3::Color;

use std::io::Write;

fn main() {
    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for i in (0..HEIGHT).rev() {
        eprint!("\rremain {}", i);
        std::io::stdout().flush().unwrap();
        for j in 0..WIDTH {
            let c: Color = Color {
                x: f64::from(j) / f64::from(WIDTH - 1),
                y: f64::from(i) / f64::from(HEIGHT - 1),
                z: 0.25,
            };
            write_color(c);
        }
    }
    eprintln!("DONE");
}
