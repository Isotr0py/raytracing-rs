use crate::interval::Interval;
use crate::vector::Vec3;

pub fn write_color(color: Vec3) {
    let intensity = Interval::new(0.000, 0.999);
    let ir = (255.999 * intensity.clamp(color.x())) as i32;
    let ig = (255.999 * intensity.clamp(color.y())) as i32;
    let ib = (255.999 * intensity.clamp(color.z())) as i32;

    println!("{} {} {}", ir, ig, ib);
}
