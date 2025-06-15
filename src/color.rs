use super::vec3::Vec3;
use std::io::Write;

pub fn write_color<W: Write>(out: &mut W, pixel_color: &Vec3) -> std::io::Result<()> {
    let r = (255.999 as f64 * pixel_color.0) as u8;
    let g = (255.999 as f64 * pixel_color.1) as u8;
    let b = (255.999 as f64 * pixel_color.2) as u8;

    writeln!(out, "{} {} {}", r, g, b)
}
