use crate::vec3::Color;

pub fn write_color(pixcel_color: Color, sample_per_pixcel: u32) {
    let e = pixcel_color / (sample_per_pixcel as f64);
    let e = e.clamp(0.0, 1.0);
    let e = 256.0 * e;

    println!("{} {} {}", e.x, e.y, e.z);
}
