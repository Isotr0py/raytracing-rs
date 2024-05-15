use camera::Camera;
use hittable_list::HittableList;
use material::{Material, Lambertian};
use sphere::Sphere;
use vector::Vec3;

mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vector;

fn main() {
    // Image
    let aspect_ratio: f64 = 3.0 / 2.0;
    let image_width: usize = 600;
    let samples_per_pixel: usize = 10;
    let max_depth: usize = 50;
    let vfov: f64 = 20.0;

    // Material
    let material_ground = Material::lambertian(Vec3::from_xyz(0.8, 0.8, 0.0));
    let material_center = Material::lambertian(Vec3::from_xyz(0.1, 0.2, 0.5));
    let material_left = Material::dielectric(1.50);
    let material_bubble = Material::dielectric(1.0/1.5);
    let material_right = Material::metal(Vec3::from_xyz(0.8, 0.6, 0.2), 1.0);

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Vec3::from_xyz(0., -100.5, -1.), 100., material_ground));
    world.add(Sphere::new(Vec3::from_xyz(0., 0., -1.2), 0.5, material_center));
    world.add(Sphere::new(Vec3::from_xyz(-1., 0., -1.), 0.5, material_left));
    world.add(Sphere::new(Vec3::from_xyz(-1., 0., -1.), 0.4, material_bubble));
    world.add(Sphere::new(Vec3::from_xyz(1., 0., -1.), 0.5, material_right));

    // Camera
    let cam = Camera::new(image_width, aspect_ratio, vfov, samples_per_pixel, max_depth);
    cam.render(&world);
}
