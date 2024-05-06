use std::f64::INFINITY;
use std::io::{self, Write};

use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vector::Vec3;

mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vector;

fn ray_color(ray: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::new();
    let color = match world.hit(ray, Interval::new(0., INFINITY), &mut rec) {
        true => 0.5 * (rec.normal + Vec3::ones()),
        false => {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            Vec3::ones() * (1.0 - t) + Vec3::from_rgb(0.5, 0.7, 1.0) * t
        }
    };
    color
}

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: usize = 600;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::from_xyz(0., 0., -1.), 0.5));
    world.add(Sphere::new(Vec3::from_xyz(0., -100.5, -1.), 100.));

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
            let color = ray_color(&ray, &world);
            color::write_color(color);
        }
    }
    eprintln!("\nDone.")
}
