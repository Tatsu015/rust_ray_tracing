fn main() {
    const WIDTH: i32 = 256;
    const HEIGHT: i32 = 256;

    println!("P3\n{} {}\n255", WIDTH, HEIGHT);

    for i in (0..HEIGHT).rev() {
        for j in 0..WIDTH {
            let r: f64 = f64::from(j) / f64::from(WIDTH - 1);
            let g: f64 = f64::from(i) / f64::from(HEIGHT - 1);
            let b: f64 = 0.25;

            let ir = (256.0 * r).floor();
            let ig = (256.0 * g).floor();
            let ib = (256.0 * b).floor();

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
