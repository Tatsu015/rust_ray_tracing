use crate::vec3::Color;

pub fn write_color(pixcel_color: Color) {
    let ir = (256.0 * pixcel_color.x).floor();
    let ig = (256.0 * pixcel_color.y).floor();
    let ib = (256.0 * pixcel_color.z).floor();

    println!("{} {} {}", ir, ig, ib);
}
