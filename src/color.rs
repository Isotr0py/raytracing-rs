use crate::interval::Interval;
use crate::vector::Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    match linear_component > 0. {
        true => linear_component.sqrt(),
        false => 0.,
    }
}

pub fn write_color(color: Vec3) {
    let intensity = Interval::new(0.000, 0.999);
    
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());

    let ir = (255.999 * intensity.clamp(r)) as i32;
    let ig = (255.999 * intensity.clamp(g)) as i32;
    let ib = (255.999 * intensity.clamp(b)) as i32;

    println!("{} {} {}", ir, ig, ib);
}
