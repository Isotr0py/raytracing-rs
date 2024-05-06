use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vector::Vec3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vector;

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: usize = 300;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::from_xyz(0., 0., -1.), 0.5));
    world.add(Sphere::new(Vec3::from_xyz(0., -100.5, -1.), 100.));

    // Camera
    let cam = Camera::new(image_width, aspect_ratio);
    cam.render(&world);
}
