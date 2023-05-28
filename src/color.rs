use crate::vec3::Color;

pub fn write_color(pixcel_color: Color, sample_per_pixcel: u32) {
    let normalize = pixcel_color / (sample_per_pixcel as f64);
    let gammad = normalize.sqrt();
    let clamped = gammad.clamp(0.0, 1.0);
    let color = 256.0 * clamped;

    println!("{} {} {}", color.x, color.y, color.z);
}
