use std::io::{self, Write};

use crate::ray::Ray;
use crate::vector::Vec3;

mod color;
mod ray;
mod vector;

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4. * a * c;
    let t = match discriminant < 0. {
        true => -1.,
        false => (-b - discriminant.sqrt()) / (2.0 * a),
    };
    t
}

fn ray_color(ray: &Ray) -> Vec3 {
    let mut t = hit_sphere(Vec3::from_xyz(0., 0., -1.), 0.5, ray);
    let color = match t > 0. {
        true => {
            let n = (ray.at(t) - Vec3::from_xyz(0., 0., -1.)).unit_vector();
            0.5 * (n + Vec3::from_rgb(1., 1., 1.))
        }
        false => {
            let unit_direction = ray.direction().unit_vector();
            t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::from_rgb(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::from_rgb(0.5, 0.7, 1.0) * t
        }
    };
    color
}

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: usize = 600;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::from_xyz(0., 0., 0.);
    let horizontal = Vec3::from_xyz(viewport_width, 0., 0.);
    let vertical = Vec3::from_xyz(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::from_xyz(0., 0., focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        io::stdout().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, direction);
            let color = ray_color(&ray);
            color::write_color(color);
        }
    }
    eprintln!("\nDone.")
}
