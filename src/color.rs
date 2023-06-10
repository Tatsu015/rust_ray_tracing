use crate::vec3::Color;

pub fn to_string(pixcel_color: Color, sample_per_pixcel: u32) -> String {
    let normalize = pixcel_color / (sample_per_pixcel as f64);
    let gammad = normalize.sqrt();
    let clamped = gammad.clamp(0.0, 1.0);
    let color = 256.0 * clamped;

    let s = format!(
        "{} {} {}\n",
        color.x.round(),
        color.y.round(),
        color.z.round()
    );
    return s;
}
